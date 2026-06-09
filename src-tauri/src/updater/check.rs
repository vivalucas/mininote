use super::{
    errors, manifest,
    platform::{self, PlatformInfo},
    settings::{self, StoredUpdateSettings},
    state,
    types::{
        DownloadSourcePreference, DownloadSourceUsed, UpdateCheckResult, UpdateCheckStatus,
        UpdateErrorDto, UpdateStateDto, UpdateStatus,
    },
    version, UpdatePaths,
};
use crate::services::notes::AppError;
use chrono::Utc;
use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
    time::Duration,
};

const GITHUB_MANIFEST_PATH_ENV: &str = "MININOTE_UPDATE_GITHUB_MANIFEST_PATH";
const GITHUB_REPO_ENV: &str = "MININOTE_UPDATE_GITHUB_REPO";
const DEFAULT_GITHUB_REPO: &str = "vivalucas/mininote";

macro_rules! debug_log {
    ($($arg:tt)*) => { super::debug_log!("check", $($arg)*) };
}

#[derive(Debug, Clone)]
struct UpdateCheckContext {
    platform: PlatformInfo,
    current_version: Version,
    allow_prerelease: bool,
    previous_state: UpdateStateDto,
}

impl UpdateCheckContext {
    fn current_version_text(&self) -> String {
        self.current_version.to_string()
    }
}

#[derive(Debug, Clone)]
struct UpdateCandidate {
    priority: usize,
    version: String,
    normalized_version: Version,
    release_notes: Option<String>,
    mandatory: bool,
    asset_name: String,
    asset_sha256: Option<String>,
    asset_size: u64,
    asset_url: Option<String>,
    github_asset_url: Option<String>,
    can_download_from_github: bool,
}

impl UpdateCandidate {
    fn asset_url_for_source(&self, source: Option<&DownloadSourceUsed>) -> Option<String> {
        match source {
            Some(DownloadSourceUsed::Github) => self.github_asset_url.clone(),
            None => None,
        }
        .or_else(|| self.asset_url.clone())
        .or_else(|| self.github_asset_url.clone())
    }
}

#[derive(Debug, Clone)]
enum ProviderCheck {
    NotAvailable,
    Available(Box<UpdateCandidate>),
}

trait UpdateCheckProvider {
    fn label(&self) -> &'static str;
    fn check(
        &self,
        context: &UpdateCheckContext,
        priority: usize,
    ) -> Result<ProviderCheck, AppError>;
}

#[derive(Debug, Clone, Default)]
struct GithubProvider {
    manifest_path: Option<PathBuf>,
    offline: bool,
}

impl GithubProvider {
    pub fn from_env() -> Self {
        Self {
            manifest_path: env_manifest_path(GITHUB_MANIFEST_PATH_ENV),
            offline: update_offline_from_env(),
        }
    }

    #[cfg(test)]
    fn with_manifest_path(path: PathBuf) -> Self {
        Self {
            manifest_path: Some(path),
            offline: false,
        }
    }

    #[cfg(test)]
    fn offline() -> Self {
        Self {
            manifest_path: None,
            offline: true,
        }
    }
}

impl UpdateCheckProvider for GithubProvider {
    fn label(&self) -> &'static str {
        "GitHub"
    }

    fn check(
        &self,
        context: &UpdateCheckContext,
        priority: usize,
    ) -> Result<ProviderCheck, AppError> {
        if let Some(manifest_path) = &self.manifest_path {
            return load_manifest_candidate(self.label(), manifest_path, context, priority);
        }

        if self.offline {
            return Err(errors::provider_not_configured(self.label()));
        }

        check_github_api(context, priority)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct UpdateCheckService {
    github: GithubProvider,
    platform_override: Option<PlatformInfo>,
}

impl UpdateCheckService {
    pub(crate) fn from_env() -> Self {
        Self {
            github: GithubProvider::from_env(),
            platform_override: None,
        }
    }

    pub(crate) fn run(
        &self,
        paths: &UpdatePaths,
        _manual: bool,
        current_version: &str,
    ) -> Result<UpdateCheckResult, AppError> {
        debug_log!("开始检查更新 当前版本=v{current_version}");
        let settings = settings::load(paths)?;
        debug_log!(
            "更新设置已加载 channel={:?} allow_prerelease={}",
            settings.channel,
            settings.allow_prerelease
        );
        let previous_state = state::load_with_current_version(paths, current_version)?;
        let context = UpdateCheckContext {
            platform: self.current_platform(current_version),
            current_version: version::normalize_version(current_version)?,
            allow_prerelease: version::allows_prerelease(
                &settings.channel,
                settings.allow_prerelease,
            ),
            previous_state,
        };
        debug_log!(
            "平台检查 os={:?} arch={:?} install_kind={:?}",
            context.platform.os,
            context.platform.arch,
            context.platform.install_kind
        );
        if let Err(error) = context.platform.ensure_in_app_updates_supported() {
            persist_last_auto_check_at(paths, &settings)?;
            state::save(paths, &failed_state(&context, &settings, &error))?;
            return Err(error);
        }

        let outcome = self.evaluate(&settings, &context);
        match outcome {
            Ok((result, next_state)) => {
                debug_log!(
                    "检查完成 status={:?} latest_version={:?}",
                    result.status,
                    result.latest_version
                );
                persist_last_auto_check_at(paths, &settings)?;
                state::save(paths, &next_state)?;
                Ok(result)
            }
            Err(error) => {
                debug_log!("检查失败 code={} message={}", error.code, error.message);
                persist_last_auto_check_at(paths, &settings)?;
                state::save(paths, &failed_state(&context, &settings, &error))?;
                Err(error)
            }
        }
    }

    #[cfg(test)]
    fn with_provider(github: GithubProvider) -> Self {
        Self {
            github,
            platform_override: None,
        }
    }

    #[cfg(test)]
    fn with_provider_and_platform(github: GithubProvider, platform: PlatformInfo) -> Self {
        Self {
            github,
            platform_override: Some(platform),
        }
    }

    fn current_platform(&self, current_version: &str) -> PlatformInfo {
        self.platform_override
            .clone()
            .unwrap_or_else(|| platform::current_platform_with_version(current_version.to_string()))
    }

    fn evaluate(
        &self,
        settings: &StoredUpdateSettings,
        context: &UpdateCheckContext,
    ) -> Result<(UpdateCheckResult, UpdateStateDto), AppError> {
        debug_log!("提供者检查源偏好={:?}", settings.check_source_preference);
        let mut available = Vec::new();
        let mut saw_not_available = false;
        let mut provider_errors = Vec::new();

        let source = DownloadSourceUsed::Github;
        let provider_result = self.github.check(context, 0);

        match provider_result {
            Ok(ProviderCheck::Available(candidate)) => {
                debug_log!("提供者 {source:?} 返回可用版本={}", candidate.version);
                available.push(*candidate);
            }
            Ok(ProviderCheck::NotAvailable) => {
                debug_log!("提供者 {source:?} 返回无更新");
                saw_not_available = true;
            }
            Err(error) => {
                debug_log!(
                    "提供者 {source:?} 返回错误 code={} message={}",
                    error.code,
                    error.message
                );
                provider_errors.push(error);
            }
        }

        debug_log!("可用候选数={}", available.len());
        if let Some(candidate) = merge_candidates(available) {
            debug_log!(
                "合并后版本={} 推荐源={:?}",
                candidate.version,
                recommended_source(
                    &settings.download_source_preference,
                    candidate.can_download_from_github,
                )
            );
            let recommended_source = recommended_source(
                &settings.download_source_preference,
                candidate.can_download_from_github,
            );
            let asset_url = candidate.asset_url_for_source(recommended_source.as_ref());
            let result = UpdateCheckResult {
                status: UpdateCheckStatus::Available,
                current_version: context.current_version_text(),
                latest_version: Some(candidate.version.clone()),
                release_notes: candidate.release_notes.clone(),
                mandatory: candidate.mandatory,
                can_download_from_github: candidate.can_download_from_github,
                recommended_source: recommended_source.clone(),
                asset_url: asset_url.clone(),
            };
            let next_state = UpdateStateDto {
                status: UpdateStatus::Available,
                current_version: context.current_version_text(),
                latest_version: Some(candidate.version),
                channel: settings.channel.clone(),
                asset_name: Some(candidate.asset_name),
                asset_path: None,
                asset_sha256: candidate.asset_sha256,
                asset_size: Some(candidate.asset_size),
                asset_url,
                source: recommended_source,
                checked_at: Some(Utc::now()),
                downloaded_at: None,
                install_log_path: None,
                install_mode: None,
                install_started_at: None,
                install_scheduled_at: None,
                last_error: None,
            };
            return Ok((result, next_state));
        }

        if saw_not_available {
            let result = UpdateCheckResult {
                status: UpdateCheckStatus::NotAvailable,
                current_version: context.current_version_text(),
                latest_version: None,
                release_notes: None,
                mandatory: false,
                can_download_from_github: false,
                recommended_source: None,
                asset_url: None,
            };
            let next_state = UpdateStateDto {
                status: UpdateStatus::Idle,
                current_version: context.current_version_text(),
                latest_version: None,
                channel: settings.channel.clone(),
                asset_name: None,
                asset_path: None,
                asset_sha256: None,
                asset_size: None,
                asset_url: None,
                source: None,
                checked_at: Some(Utc::now()),
                downloaded_at: None,
                install_log_path: None,
                install_mode: None,
                install_started_at: None,
                install_scheduled_at: None,
                last_error: None,
            };
            return Ok((result, next_state));
        }

        Err(aggregate_provider_errors(provider_errors))
    }
}

pub(super) fn env_manifest_path(key: &str) -> Option<PathBuf> {
    env::var_os(key).and_then(|value| {
        let value = value.to_string_lossy().trim().to_string();
        (!value.is_empty()).then(|| PathBuf::from(value))
    })
}

fn update_offline_from_env() -> bool {
    env::var("MININOTE_UPDATE_OFFLINE").is_ok()
}

fn github_repo() -> String {
    env::var(GITHUB_REPO_ENV)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| DEFAULT_GITHUB_REPO.to_string())
}

fn build_github_api_client() -> Result<Client, AppError> {
    Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(15))
        .user_agent("mininote-updater")
        .build()
        .map_err(|error| errors::github_api_error(format!("无法创建 HTTP 客户端：{error}")))
}

#[derive(Debug, Deserialize)]
struct GithubApiAsset {
    name: String,
    browser_download_url: String,
    size: u64,
    #[serde(default)]
    digest: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GithubApiRelease {
    tag_name: String,
    #[allow(dead_code)]
    name: Option<String>,
    body: Option<String>,
    assets: Vec<GithubApiAsset>,
}

fn fetch_latest_github_release() -> Result<GithubApiRelease, AppError> {
    let repo = github_repo();
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");

    let client = build_github_api_client()?;
    let response = client.get(&url).send().map_err(|error| {
        if error.is_timeout() {
            errors::github_api_error("请求超时")
        } else {
            errors::github_api_error(error.to_string())
        }
    })?;

    let status = response.status();
    if status.as_u16() == 403 || status.as_u16() == 429 {
        return Err(errors::github_rate_limited());
    }
    if !status.is_success() {
        return Err(errors::github_api_error(format!(
            "HTTP {}",
            status.as_u16()
        )));
    }

    let body = response
        .text()
        .map_err(|error| errors::github_api_error(format!("响应读取失败：{error}")))?;
    serde_json::from_str(&body)
        .map_err(|error| errors::github_api_error(format!("响应解析失败：{error}")))
}

pub(crate) struct GithubDownloadInfo {
    pub asset_name: String,
    pub asset_size: u64,
    pub url: String,
    pub sha256: Option<String>,
}

pub(crate) fn fetch_github_download_info(
    platform: &PlatformInfo,
    version: &str,
    asset_name: &str,
    expected_size: Option<u64>,
) -> Result<GithubDownloadInfo, AppError> {
    let release = fetch_latest_github_release()?;
    let release_version = release
        .tag_name
        .trim_start_matches('v')
        .trim_start_matches('V');
    if version::normalize_version(release_version)? != version::normalize_version(version)? {
        return Err(errors::with_detail(
            errors::app_error(
                "updateDownloadVersionMismatch",
                "GitHub 最新 Release 与当前待下载版本不一致",
            ),
            "expectedVersion",
            version,
        ));
    }

    let matched = release
        .assets
        .iter()
        .find(|asset| asset.name == asset_name)
        .or_else(|| {
            release.assets.iter().find(|asset| {
                platform::infer_asset_from_filename(
                    &asset.name,
                    &asset.browser_download_url,
                    asset.size,
                )
                .is_some_and(|inferred| {
                    inferred.os == platform.os
                        && inferred.arch == platform.arch
                        && matches_install_kind(&inferred.kind, &platform.install_kind)
                })
            })
        })
        .ok_or_else(|| {
            errors::with_detail(errors::manifest_asset_not_found(), "assetName", asset_name)
        })?;

    if expected_size.is_some_and(|size| size > 0 && matched.size != size) {
        return Err(errors::with_detail(
            errors::app_error(
                "updateDownloadAssetMismatch",
                "GitHub Release 中的更新包元数据与已检查结果不一致",
            ),
            "assetName",
            asset_name,
        ));
    }

    Ok(GithubDownloadInfo {
        asset_name: matched.name.clone(),
        asset_size: matched.size,
        url: matched.browser_download_url.clone(),
        sha256: matched.digest.as_deref().and_then(parse_sha256_from_digest),
    })
}

fn check_github_api(
    context: &UpdateCheckContext,
    priority: usize,
) -> Result<ProviderCheck, AppError> {
    debug_log!("GitHub API 请求 latest release");
    let release = fetch_latest_github_release()?;

    let version_str = release
        .tag_name
        .trim_start_matches('v')
        .trim_start_matches('V');
    let normalized_version = version::normalize_version(version_str)?;
    debug_log!(
        "GitHub Release tag={} assets数={}",
        release.tag_name,
        release.assets.len()
    );

    if !version::is_newer_version(
        &context.current_version,
        &normalized_version,
        context.allow_prerelease,
    ) {
        return Ok(ProviderCheck::NotAvailable);
    }

    if release.assets.is_empty() {
        return Err(errors::github_release_no_assets());
    }

    let matched_api_asset = release
        .assets
        .iter()
        .find(|asset| {
            platform::infer_asset_from_filename(
                &asset.name,
                &asset.browser_download_url,
                asset.size,
            )
            .is_some_and(|inferred| {
                inferred.os == context.platform.os
                    && inferred.arch == context.platform.arch
                    && matches_install_kind(&inferred.kind, &context.platform.install_kind)
            })
        })
        .ok_or_else(|| {
            errors::with_detail(
                errors::manifest_asset_not_found(),
                "platform",
                format!(
                    "{:?}-{:?}-{:?}",
                    context.platform.os, context.platform.arch, context.platform.install_kind
                ),
            )
        })?;

    let asset_sha256 = matched_api_asset
        .digest
        .as_deref()
        .and_then(parse_sha256_from_digest);

    Ok(ProviderCheck::Available(Box::new(UpdateCandidate {
        priority,
        version: version_str.to_string(),
        normalized_version,
        release_notes: release.body,
        mandatory: false,
        asset_name: matched_api_asset.name.clone(),
        asset_sha256,
        asset_size: matched_api_asset.size,
        asset_url: Some(matched_api_asset.browser_download_url.clone()),
        github_asset_url: Some(matched_api_asset.browser_download_url.clone()),
        can_download_from_github: true,
    })))
}

/// Parses `"sha256:<hex>"` → `Some("<hex>")`, returns `None` for other formats.
fn parse_sha256_from_digest(digest: &str) -> Option<String> {
    let hex = digest.strip_prefix("sha256:")?;
    if hex.len() == 64 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        Some(hex.to_ascii_lowercase())
    } else {
        None
    }
}

fn matches_install_kind(
    inferred: &super::types::InstallKind,
    current: &super::types::InstallKind,
) -> bool {
    *current == super::types::InstallKind::Unknown || inferred == current
}

fn persist_last_auto_check_at(
    paths: &UpdatePaths,
    settings: &StoredUpdateSettings,
) -> Result<(), AppError> {
    let mut settings = settings.clone();
    settings.last_auto_check_at = Some(Utc::now());
    settings::save(paths, &settings)
}

fn load_manifest_candidate(
    provider: &str,
    manifest_path: &Path,
    context: &UpdateCheckContext,
    priority: usize,
) -> Result<ProviderCheck, AppError> {
    let manifest_bytes = fs::read(manifest_path).map_err(|error| {
        let error = errors::with_detail(
            errors::app_error(
                "updateProviderFixtureUnreadable",
                format!("无法读取 {provider} 更新测试清单：{error}"),
            ),
            "provider",
            provider,
        );
        errors::with_detail(error, "path", manifest_path.display().to_string())
    })?;
    let manifest = manifest::parse_manifest(&manifest_bytes)?;
    let asset = manifest::select_asset(
        &manifest,
        &context.platform,
        context.platform.install_kind.clone(),
    )?;
    let candidate_version = manifest.normalized_version()?;
    if !version::is_newer_version(
        &context.current_version,
        &candidate_version,
        context.allow_prerelease,
    ) {
        return Ok(ProviderCheck::NotAvailable);
    }

    let github_asset_url = (!asset.github_url.trim().is_empty()).then(|| asset.github_url.clone());
    let has_github_url = github_asset_url.is_some();

    Ok(ProviderCheck::Available(Box::new(UpdateCandidate {
        priority,
        version: manifest.version.clone(),
        normalized_version: candidate_version,
        release_notes: manifest.release_notes.clone(),
        mandatory: manifest.mandatory,
        asset_name: asset.name.clone(),
        asset_sha256: Some(asset.sha256),
        asset_size: asset.size,
        asset_url: github_asset_url.clone(),
        github_asset_url,
        can_download_from_github: has_github_url,
    })))
}

fn merge_candidates(mut candidates: Vec<UpdateCandidate>) -> Option<UpdateCandidate> {
    debug_log!("合并候选版本 候选数={}", candidates.len());
    if candidates.is_empty() {
        return None;
    }

    candidates.sort_by(|left, right| {
        right
            .normalized_version
            .cmp(&left.normalized_version)
            .then(left.priority.cmp(&right.priority))
    });

    let best_version = candidates.first()?.normalized_version.clone();
    let mut matching_candidates = candidates
        .into_iter()
        .filter(|candidate| candidate.normalized_version == best_version)
        .collect::<Vec<_>>();
    matching_candidates.sort_by_key(|candidate| candidate.priority);

    let mut primary = matching_candidates.remove(0);
    let fallback_candidates = matching_candidates;

    primary.can_download_from_github |= fallback_candidates
        .iter()
        .any(|candidate| candidate.can_download_from_github);
    primary.mandatory |= fallback_candidates
        .iter()
        .any(|candidate| candidate.mandatory);

    if primary.github_asset_url.is_none() {
        primary.github_asset_url = fallback_candidates
            .iter()
            .find_map(|candidate| candidate.github_asset_url.clone());
    }
    if primary.asset_sha256.is_none() {
        primary.asset_sha256 = fallback_candidates
            .iter()
            .find_map(|candidate| candidate.asset_sha256.clone());
    }
    if primary.asset_size == 0 {
        if let Some(candidate) = fallback_candidates
            .iter()
            .find(|candidate| candidate.asset_size > 0)
        {
            primary.asset_size = candidate.asset_size;
            primary.asset_name = candidate.asset_name.clone();
        }
    }

    if primary
        .release_notes
        .as_deref()
        .unwrap_or("")
        .trim()
        .is_empty()
    {
        primary.release_notes = fallback_candidates.iter().find_map(|candidate| {
            candidate
                .release_notes
                .clone()
                .filter(|notes| !notes.trim().is_empty())
        });
    }

    Some(primary)
}

fn recommended_source(
    preference: &DownloadSourcePreference,
    can_download_from_github: bool,
) -> Option<DownloadSourceUsed> {
    match preference {
        DownloadSourcePreference::GithubFirst => {
            if can_download_from_github {
                Some(DownloadSourceUsed::Github)
            } else {
                None
            }
        }
    }
}

fn aggregate_provider_errors(errors_list: Vec<AppError>) -> AppError {
    if errors_list.is_empty() {
        return errors::source_not_configured();
    }

    if errors_list
        .iter()
        .all(|error| error.code == "updateProviderNotConfigured")
    {
        let providers = errors_list
            .iter()
            .filter_map(|error| error.details.get("provider"))
            .cloned()
            .collect::<Vec<_>>()
            .join(",");
        let error = errors::source_not_configured();
        return if providers.is_empty() {
            error
        } else {
            errors::with_detail(error, "providers", providers)
        };
    }

    errors_list
        .into_iter()
        .find(|error| error.code != "updateProviderNotConfigured")
        .unwrap_or_else(errors::source_not_configured)
}

fn failed_state(
    context: &UpdateCheckContext,
    settings: &StoredUpdateSettings,
    error: &AppError,
) -> UpdateStateDto {
    UpdateStateDto {
        status: UpdateStatus::Failed,
        current_version: context.current_version_text(),
        latest_version: context.previous_state.latest_version.clone(),
        channel: settings.channel.clone(),
        asset_name: context.previous_state.asset_name.clone(),
        asset_path: context.previous_state.asset_path.clone(),
        asset_sha256: context.previous_state.asset_sha256.clone(),
        asset_size: context.previous_state.asset_size,
        asset_url: context.previous_state.asset_url.clone(),
        source: context.previous_state.source.clone(),
        checked_at: Some(Utc::now()),
        downloaded_at: context.previous_state.downloaded_at,
        install_log_path: context.previous_state.install_log_path.clone(),
        install_mode: context.previous_state.install_mode.clone(),
        install_started_at: context.previous_state.install_started_at,
        install_scheduled_at: context.previous_state.install_scheduled_at,
        last_error: Some(UpdateErrorDto::recoverable(
            error.code.clone(),
            error.message.clone(),
            update_error_action(error).map(str::to_string),
        )),
    }
}

fn update_error_action(error: &AppError) -> Option<&'static str> {
    match error.code.as_str() {
        "updateSourceNotConfigured" | "updateProviderNotConfigured" => {
            Some("configureUpdateSource")
        }
        "updateProviderFixtureUnreadable" => Some("fixFixturePath"),
        "updatePlatformUnsupported" | "updatePortableManualOnly" => Some("useSupportedInstall"),
        "updateGithubApi" | "updateGithubRateLimited" | "updateGithubNoAssets" => Some("retry"),
        _ => Some("retry"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::updater::{
        platform::{Arch, Os},
        types::{CheckSourcePreference, InstallKind},
        UpdatePaths,
    };

    const VALID_MANIFEST_BYTES: &[u8] = include_bytes!("fixtures/update-manifest.valid.json");

    fn test_paths(name: &str) -> UpdatePaths {
        let root = std::env::temp_dir()
            .join("mininote-updater-tests")
            .join(name);
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove stale test dir");
        }
        UpdatePaths::new(root)
    }

    fn test_context(install_kind: InstallKind) -> UpdateCheckContext {
        UpdateCheckContext {
            platform: test_platform(Os::Macos, Arch::Aarch64, install_kind),
            current_version: Version::new(1, 0, 3),
            allow_prerelease: false,
            previous_state: UpdateStateDto::idle_with_version("1.0.3"),
        }
    }

    fn test_platform(os: Os, arch: Arch, install_kind: InstallKind) -> PlatformInfo {
        PlatformInfo {
            os,
            arch,
            app_version: "1.0.3".into(),
            app_id: super::super::APP_ID.into(),
            install_kind,
            current_exe: None,
            current_app_bundle: None,
        }
    }

    fn test_settings(preference: CheckSourcePreference) -> StoredUpdateSettings {
        StoredUpdateSettings {
            download_source_preference: DownloadSourcePreference::GithubFirst,
            check_source_preference: preference,
            channel: super::super::types::UpdateChannel::Stable,
            ..StoredUpdateSettings::default()
        }
    }

    fn write_manifest(paths: &UpdatePaths, name: &str, version: &str) -> PathBuf {
        paths.ensure_dirs().expect("create test dirs");
        let raw = std::str::from_utf8(VALID_MANIFEST_BYTES)
            .expect("fixture utf8")
            .replace("1.0.5", version);
        let path = paths.root_dir().join(name);
        fs::write(&path, raw).expect("write manifest fixture");
        path
    }

    #[test]
    fn returns_source_not_configured_when_no_provider_fixture_exists_and_github_only() {
        let service = UpdateCheckService::with_provider_and_platform(
            GithubProvider::offline(),
            test_platform(Os::Macos, Arch::Aarch64, InstallKind::MacosAppBundle),
        );
        let settings = test_settings(CheckSourcePreference::GithubFirst);

        let result = service.evaluate(&settings, &test_context(InstallKind::MacosAppBundle));
        assert!(result.is_err());
    }

    #[test]
    fn prefers_highest_available_version_across_providers() {
        let paths = test_paths("check-highest-version");
        let github_manifest = write_manifest(&paths, "github.json", "1.0.6");
        let service =
            UpdateCheckService::with_provider(GithubProvider::with_manifest_path(github_manifest));
        let settings = test_settings(CheckSourcePreference::GithubFirst);

        let (result, next_state) = service
            .evaluate(&settings, &test_context(InstallKind::MacosAppBundle))
            .expect("configured manifests should return result");

        assert_eq!(result.status, UpdateCheckStatus::Available);
        assert_eq!(result.latest_version.as_deref(), Some("1.0.6"));
        assert_eq!(result.recommended_source, Some(DownloadSourceUsed::Github));
        assert_eq!(next_state.status, UpdateStatus::Available);
        assert_eq!(next_state.latest_version.as_deref(), Some("1.0.6"));
    }

    #[test]
    fn returns_not_available_when_candidate_is_not_newer() {
        let paths = test_paths("check-not-available");
        let github_manifest = write_manifest(&paths, "github.json", "1.0.3");
        let service =
            UpdateCheckService::with_provider(GithubProvider::with_manifest_path(github_manifest));
        let settings = test_settings(CheckSourcePreference::GithubFirst);

        let (result, next_state) = service
            .evaluate(&settings, &test_context(InstallKind::MacosAppBundle))
            .expect("matching version should not error");

        assert_eq!(result.status, UpdateCheckStatus::NotAvailable);
        assert_eq!(next_state.status, UpdateStatus::Idle);
        assert!(next_state.latest_version.is_none());
    }

    #[test]
    fn stores_asset_url_in_state_from_manifest_fixture() {
        let paths = test_paths("check-asset-url");
        let github_manifest = write_manifest(&paths, "github.json", "1.0.5");
        let service =
            UpdateCheckService::with_provider(GithubProvider::with_manifest_path(github_manifest));
        let settings = test_settings(CheckSourcePreference::GithubFirst);

        let (result, next_state) = service
            .evaluate(&settings, &test_context(InstallKind::MacosAppBundle))
            .expect("available update should have asset url");

        assert!(result.asset_url.is_some());
        assert!(next_state.asset_url.is_some());
    }

    #[test]
    fn manual_run_updates_last_auto_check_timestamp() {
        let paths = test_paths("check-manual-updates-last-auto-check-at");
        let github_manifest = write_manifest(&paths, "github.json", "1.0.5");
        let service = UpdateCheckService::with_provider_and_platform(
            GithubProvider::with_manifest_path(github_manifest),
            test_platform(Os::Macos, Arch::Aarch64, InstallKind::MacosAppBundle),
        );

        service
            .run(&paths, true, "1.0.3")
            .expect("manual check should succeed");

        let saved_settings = settings::load(&paths).expect("load settings");
        assert!(saved_settings.last_auto_check_at.is_some());
    }

    #[test]
    fn run_rejects_unknown_install_kind() {
        let paths = test_paths("check-run-unknown-platform");
        let service = UpdateCheckService::with_provider_and_platform(
            GithubProvider::offline(),
            test_platform(Os::Macos, Arch::Aarch64, InstallKind::Unknown),
        );

        let error = service
            .run(&paths, true, "1.0.3")
            .expect_err("unknown install kind should be rejected");

        assert_eq!(error.code, "updatePlatformUnsupported");
        let saved_state = state::load(&paths).expect("load failed state");
        assert_eq!(saved_state.status, UpdateStatus::Failed);
        assert_eq!(
            saved_state
                .last_error
                .as_ref()
                .and_then(|error| error.action.as_deref()),
            Some("useSupportedInstall")
        );
    }

    #[test]
    fn run_rejects_windows_portable_install_kind() {
        let paths = test_paths("check-run-portable-platform");
        let service = UpdateCheckService::with_provider_and_platform(
            GithubProvider::offline(),
            test_platform(Os::Windows, Arch::X86_64, InstallKind::WindowsPortable),
        );

        let error = service
            .run(&paths, true, "1.0.3")
            .expect_err("portable install kind should be rejected");

        assert_eq!(error.code, "updatePortableManualOnly");
        let saved_state = state::load(&paths).expect("load failed state");
        assert_eq!(saved_state.status, UpdateStatus::Failed);
        assert_eq!(
            saved_state
                .last_error
                .as_ref()
                .and_then(|error| error.action.as_deref()),
            Some("useSupportedInstall")
        );
    }

    #[test]
    fn run_preserves_previous_available_update_when_check_fails() {
        let paths = test_paths("check-preserve-available-on-failure");
        let mut previous = UpdateStateDto::idle_with_version("1.0.3");
        previous.status = UpdateStatus::Available;
        previous.latest_version = Some("1.0.5".into());
        previous.asset_name = Some("mininote_1.0.5_macos_aarch64_app.zip".into());
        previous.asset_sha256 =
            Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into());
        previous.asset_size = Some(42);
        previous.source = Some(DownloadSourceUsed::Github);
        state::save(&paths, &previous).expect("seed available state");

        let service = UpdateCheckService::with_provider_and_platform(
            GithubProvider::offline(),
            test_platform(Os::Macos, Arch::Aarch64, InstallKind::MacosAppBundle),
        );

        let error = service
            .run(&paths, false, "1.0.3")
            .expect_err("unconfigured providers should fail");

        assert_eq!(error.code, "updateSourceNotConfigured");
        let saved_state = state::load(&paths).expect("load failed state");
        assert_eq!(saved_state.status, UpdateStatus::Failed);
        assert_eq!(saved_state.latest_version.as_deref(), Some("1.0.5"));
        assert_eq!(
            saved_state.asset_name.as_deref(),
            Some("mininote_1.0.5_macos_aarch64_app.zip")
        );
        assert_eq!(saved_state.asset_size, Some(42));
    }
}
