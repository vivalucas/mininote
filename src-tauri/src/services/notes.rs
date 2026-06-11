use crate::json_io::{write_json_atomic, write_text_atomic};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env, fmt, fs, io,
    path::{Component, Path, PathBuf},
};
use uuid::Uuid;

#[cfg(target_os = "macos")]
const DEFAULT_MACOS_GLOBAL_SHORTCUT: &str = "Command+Option+N";
const MAX_IMAGE_SIZE: usize = 30 * 1024 * 1024;
const MAX_IMPORT_TEXT_SIZE: u64 = 10 * 1024 * 1024;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(default = "default_locale")]
    pub locale: String,
    pub notes_dir: String,
    pub global_shortcut: String,
    pub close_to_tray: bool,
    pub autostart: bool,
    pub default_view_mode: String,
    #[serde(default = "default_note_auto_save")]
    pub note_auto_save: bool,
    #[serde(default = "default_note_surface_auto_save")]
    pub note_surface_auto_save: bool,
    #[serde(default = "default_tile_color")]
    pub tile_color: String,
    #[serde(default = "default_tile_color_mode")]
    pub tile_color_mode: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_font_size")]
    pub font_size: u32,
    #[serde(default = "default_surface_font_size")]
    pub surface_font_size: u32,
    #[serde(default = "default_tab_indent_size")]
    pub tab_indent_size: u32,
    #[serde(default = "default_external_file_auto_save")]
    pub external_file_auto_save: bool,
    #[serde(default)]
    pub background_image_path: String,
    #[serde(default = "default_background_fit")]
    pub background_fit: String,
    #[serde(default = "default_background_dim")]
    pub background_dim: f64,
    #[serde(default = "default_background_blur")]
    pub background_blur: f64,
    #[serde(default = "default_background_scale")]
    pub background_scale: f64,
    #[serde(default = "default_background_position")]
    pub background_position_x: f64,
    #[serde(default = "default_background_position")]
    pub background_position_y: f64,
    #[serde(default = "default_remember_surface_size")]
    pub remember_surface_size: bool,
    #[serde(default = "default_tile_ctrl_close")]
    pub tile_ctrl_close: bool,
    #[serde(default)]
    pub tile_render_markdown: bool,
    #[serde(default)]
    pub render_html_markdown: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface_width: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface_height: Option<u32>,
    #[serde(default = "default_toggle_visibility_shortcut")]
    pub toggle_visibility_shortcut: String,
    #[serde(default = "default_open_at_cursor")]
    pub open_at_cursor: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SaveNoteRequest {
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub category: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_modified_time: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SyncSourceRequest {
    pub content: String,
    #[serde(default)]
    pub expected_modified_time: Option<f64>,
    #[serde(default)]
    pub force: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NoteMetadata {
    pub id: String,
    pub title: String,
    pub file_name: String,
    #[serde(default)]
    pub category: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_modified_time: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub word_count: usize,
    pub preview: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    pub title: String,
    pub file_name: String,
    #[serde(default)]
    pub category: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_modified_time: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub word_count: usize,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub details: BTreeMap<String, String>,
}

impl AppError {
    fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: BTreeMap::new(),
        }
    }

    fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }

    fn note_not_found(id: &str) -> Self {
        Self::new("noteNotFound", format!("Note {id} was not found")).with_detail("noteId", id)
    }

    fn unsupported_file() -> Self {
        Self::new("unsupportedFile", "只支持导入 .mint / .md / .txt 文件")
    }

    fn file_too_large(max_mb: u64) -> Self {
        Self::new("fileTooLarge", format!("文件过大（上限 {max_mb} MB）"))
            .with_detail("maxMb", max_mb.to_string())
    }

    fn category_name_empty() -> Self {
        Self::new("categoryNameEmpty", "分类名不能为空")
    }

    fn category_name_invalid_chars() -> Self {
        Self::new("categoryNameInvalidChars", "分类名不能包含特殊字符")
    }

    fn category_not_found(name: &str) -> Self {
        Self::new("categoryNotFound", format!("分类「{name}」不存在")).with_detail("category", name)
    }

    fn category_already_exists(name: &str) -> Self {
        Self::new("categoryAlreadyExists", format!("分类「{name}」已存在"))
            .with_detail("category", name)
    }

    fn source_file_conflict(
        path: &str,
        current_modified_time: f64,
        expected_modified_time: f64,
    ) -> Self {
        Self::new("sourceFileConflict", "原文件已被其他程序修改")
            .with_detail("path", path)
            .with_detail("currentModifiedTime", current_modified_time.to_string())
            .with_detail("expectedModifiedTime", expected_modified_time.to_string())
    }

    fn source_file_missing(path: &str) -> Self {
        Self::new("sourceFileMissing", "原文件不存在或不可访问").with_detail("path", path)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::new("io", error.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::new("json", error.to_string())
    }
}

impl From<tauri::Error> for AppError {
    fn from(error: tauri::Error) -> Self {
        Self::new("tauri", error.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(super) struct MetadataFile {
    pub notes: Vec<NoteMetadata>,
}

#[derive(Debug, Clone)]
pub struct NoteStore {
    base_dir: PathBuf,
}

pub fn default_store() -> Result<NoteStore, AppError> {
    Ok(NoteStore::new(default_base_dir()?))
}

fn default_base_dir() -> Result<PathBuf, AppError> {
    if let Some(path) = configured_data_dir() {
        return Ok(path);
    }

    #[cfg(target_os = "macos")]
    if let Some(dir) = dirs::data_dir() {
        return Ok(dir.join("MiniNote"));
    }

    if let Some(dir) = dirs::document_dir() {
        return Ok(dir.join("MiniNote"));
    }

    Ok(env::current_dir()?.join("data"))
}

fn configured_data_dir() -> Option<PathBuf> {
    let Ok(path) = env::var("MININOTE_DATA_DIR") else {
        return None;
    };
    let trimmed = path.trim();
    if !trimmed.is_empty() {
        return Some(PathBuf::from(trimmed));
    }
    None
}

fn is_filesystem_root(path: &Path) -> bool {
    let path = path.to_string_lossy();
    let trimmed = path.trim_end_matches(['/', '\\']);
    if trimmed.is_empty() {
        return true;
    }
    // Windows drive root: "C:" or "D:" etc.
    if trimmed.len() == 2 {
        let bytes = trimmed.as_bytes();
        if bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
            return true;
        }
    }
    false
}

fn ensure_notes_suffix(dir: &str) -> String {
    let path = Path::new(dir);
    if path.file_name().and_then(|n| n.to_str()) == Some("notes") {
        return dir.to_string();
    }
    path.join("notes").to_string_lossy().to_string()
}

fn is_safe_notes_dir(path: &Path) -> Result<(), AppError> {
    if is_filesystem_root(path) {
        return Err(AppError::new(
            "unsafePath",
            "不能将磁盘根目录设为笔记目录，请选择一个子文件夹",
        ));
    }

    let normalized = path.to_string_lossy().to_lowercase();
    let blocked = [
        "\\windows",
        "\\program files",
        "\\program files (x86)",
        "\\system32",
        "\\syswow64",
    ];
    for suffix in &blocked {
        if normalized.ends_with(suffix) {
            return Err(AppError::new(
                "unsafePath",
                format!("不能将系统目录「{}」设为笔记目录", path.display()),
            ));
        }
    }

    // Must have at least 2 real path components (e.g. D:\Something, not just D:\)
    let real_components = path
        .components()
        .filter(|c| matches!(c, Component::Normal(_)))
        .count();
    if real_components == 0 {
        return Err(AppError::new(
            "unsafePath",
            "笔记目录路径不合法，请选择一个具体的文件夹",
        ));
    }

    Ok(())
}

impl NoteStore {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    pub fn metadata_path(&self) -> PathBuf {
        self.base_dir.join("metadata.json")
    }

    pub fn config_path(&self) -> PathBuf {
        self.base_dir.join("config.json")
    }

    pub fn load_config(&self) -> Result<AppConfig, AppError> {
        self.ensure_base_dir()?;
        let path = self.config_path();
        if !path.exists() {
            let config = self.default_config();
            self.save_config(config.clone())?;
            return Ok(config);
        }

        let mut config: AppConfig = serde_json::from_str(&fs::read_to_string(&path)?)?;
        if is_safe_notes_dir(Path::new(&config.notes_dir)).is_err() {
            config.notes_dir = self.default_config().notes_dir;
        }
        write_json_atomic(&path, &config)?;
        fs::create_dir_all(&config.notes_dir)?;
        Ok(config)
    }

    pub fn save_config(&self, mut config: AppConfig) -> Result<AppConfig, AppError> {
        self.ensure_base_dir()?;
        config.notes_dir = ensure_notes_suffix(&config.notes_dir);
        config.tab_indent_size = config.tab_indent_size.clamp(1, 8);
        is_safe_notes_dir(Path::new(&config.notes_dir))?;
        fs::create_dir_all(&config.notes_dir)?;
        write_json_atomic(&self.config_path(), &config)?;
        Ok(config)
    }

    pub fn list_notes(&self) -> Result<Vec<NoteMetadata>, AppError> {
        self.ensure_storage()?;
        let mut metadata = self.load_metadata()?.notes;
        metadata.retain(|note| {
            self.note_path_in_category(&note.file_name, &note.category)
                .exists()
        });
        metadata.sort_by_key(|note| std::cmp::Reverse(note.updated_at));
        Ok(metadata)
    }

    pub fn read_note(&self, id: &str) -> Result<Note, AppError> {
        self.ensure_storage()?;
        let metadata = self.find_metadata(id)?;
        let content = fs::read_to_string(
            self.note_path_in_category(&metadata.file_name, &metadata.category),
        )?;
        Ok(Note {
            id: metadata.id,
            title: metadata.title,
            file_name: metadata.file_name,
            category: metadata.category,
            source_path: metadata.source_path,
            source_modified_time: metadata.source_modified_time,
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
            word_count: metadata.word_count,
            content,
        })
    }

    pub fn create_note(&self, request: SaveNoteRequest) -> Result<Note, AppError> {
        self.ensure_storage()?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let file_name = self.file_name_for(&id, &request.title);
        let word_count = count_words(&request.content);
        let category = normalize_note_category(&request.category)?;
        let note_path = self.note_path_in_category(&file_name, &category);
        if let Some(parent) = note_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let metadata = NoteMetadata {
            id: id.clone(),
            title: request.title,
            file_name: file_name.clone(),
            category: category.clone(),
            source_path: request.source_path.clone(),
            source_modified_time: request.source_modified_time,
            created_at: now,
            updated_at: now,
            word_count,
            preview: preview(&request.content),
        };

        write_text_atomic(&note_path, &request.content)?;
        let mut metadata_file = self.load_metadata()?;
        metadata_file.notes.push(metadata.clone());
        self.save_metadata(&metadata_file)?;

        Ok(Note {
            id,
            title: metadata.title,
            file_name,
            category,
            source_path: metadata.source_path,
            source_modified_time: metadata.source_modified_time,
            created_at: now,
            updated_at: now,
            word_count,
            content: request.content,
        })
    }

    pub fn update_note(&self, id: &str, request: SaveNoteRequest) -> Result<Note, AppError> {
        self.ensure_storage()?;
        let mut metadata_file = self.load_metadata()?;
        let note = metadata_file
            .notes
            .iter_mut()
            .find(|note| note.id == id)
            .ok_or_else(|| AppError::note_not_found(id))?;

        let old_file_name = note.file_name.clone();
        let old_category = note.category.clone();
        let new_file_name = self.file_name_for(id, &request.title);
        let new_category = normalize_note_category(&request.category)?;
        let source_path = note.source_path.clone();
        let source_modified_time = note.source_modified_time;
        let now = Utc::now();
        let word_count = count_words(&request.content);

        let new_path = self.note_path_in_category(&new_file_name, &new_category);
        if let Some(parent) = new_path.parent() {
            fs::create_dir_all(parent)?;
        }
        write_text_atomic(&new_path, &request.content)?;

        if old_file_name != new_file_name || old_category != new_category {
            let old_path = self.note_path_in_category(&old_file_name, &old_category);
            if old_path.exists() && old_path != new_path {
                if let Err(trash_error) = trash::delete(&old_path) {
                    if let Err(remove_error) = fs::remove_file(&old_path) {
                        let _ = fs::remove_file(&new_path);
                        return Err(AppError::new(
                            "trash",
                            format!(
                                "移入回收站失败: {trash_error}; 删除旧文件也失败: {remove_error}"
                            ),
                        ));
                    }
                }
            }
        }

        note.title = request.title;
        note.file_name = new_file_name.clone();
        note.category = new_category.clone();
        note.source_path = source_path;
        note.source_modified_time = source_modified_time;
        note.updated_at = now;
        note.word_count = word_count;
        note.preview = preview(&request.content);

        let result = Note {
            id: note.id.clone(),
            title: note.title.clone(),
            file_name: note.file_name.clone(),
            category: new_category,
            source_path: note.source_path.clone(),
            source_modified_time: note.source_modified_time,
            created_at: note.created_at,
            updated_at: note.updated_at,
            word_count: note.word_count,
            content: request.content,
        };

        self.save_metadata(&metadata_file)?;
        Ok(result)
    }

    pub fn delete_note(&self, id: &str) -> Result<(), AppError> {
        self.ensure_storage()?;
        let mut metadata_file = self.load_metadata()?;
        let index = metadata_file
            .notes
            .iter()
            .position(|note| note.id == id)
            .ok_or_else(|| AppError::note_not_found(id))?;
        let metadata = metadata_file.notes.remove(index);
        let path = self.note_path_in_category(&metadata.file_name, &metadata.category);
        if path.exists() {
            trash::delete(&path)
                .map_err(|e| AppError::new("trash", format!("移入回收站失败: {e}")))?;
        }
        self.save_metadata(&metadata_file)?;
        let _ = self.delete_note_images(id);
        Ok(())
    }

    pub fn images_dir(&self, note_id: &str) -> PathBuf {
        self.base_dir.join("images").join(note_id)
    }

    pub fn save_image(
        &self,
        note_id: &str,
        data: &[u8],
        extension: &str,
    ) -> Result<String, AppError> {
        self.ensure_storage()?;
        self.find_metadata(note_id)?;

        const ALLOWED_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp", "bmp"];
        let ext = extension.to_ascii_lowercase();
        if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
            return Err(AppError::new(
                "unsupportedImageFormat",
                format!("不支持的图片格式: {ext}"),
            ));
        }
        if data.len() > MAX_IMAGE_SIZE {
            return Err(AppError::new("imageTooLarge", "图片文件过大（上限 30 MB）")
                .with_detail("maxMb", "30"));
        }

        let dir = self.images_dir(note_id);
        fs::create_dir_all(&dir)?;

        let file_name = format!("{}.{}", Uuid::new_v4(), ext);
        fs::write(dir.join(&file_name), data)?;

        Ok(format!("images/{note_id}/{file_name}"))
    }

    pub fn delete_note_images(&self, note_id: &str) -> Result<(), AppError> {
        let dir = self.images_dir(note_id);
        if dir.exists() {
            fs::remove_dir_all(&dir)?;
        }
        Ok(())
    }

    pub fn clean_unused_images(
        &self,
        note_id: &str,
        content: &str,
    ) -> Result<Vec<String>, AppError> {
        let dir = self.images_dir(note_id);
        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut removed = Vec::new();
        let mut remaining = 0usize;
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let file_name = entry.file_name().to_string_lossy().to_string();
            let relative = format!("images/{note_id}/{file_name}");
            if !content.contains(&relative) {
                fs::remove_file(&path)?;
                removed.push(file_name);
            } else {
                remaining += 1;
            }
        }

        if remaining == 0 {
            let _ = fs::remove_dir(&dir);
        }

        Ok(removed)
    }

    pub fn import_markdown_file(&self, path: &Path, category: &str) -> Result<Note, AppError> {
        if !is_supported_text_path(path) {
            return Err(AppError::unsupported_file());
        }
        let category = normalize_note_category(category)?;
        let file_size = fs::metadata(path)?.len();
        if file_size > MAX_IMPORT_TEXT_SIZE {
            return Err(AppError::file_too_large(MAX_IMPORT_TEXT_SIZE / 1024 / 1024));
        }

        let content = fs::read_to_string(path)?;
        let title = imported_markdown_title(path, &content);
        let source_modified_time = file_modified_time_ms(path).ok();
        self.create_note(SaveNoteRequest {
            title,
            content,
            category,
            source_path: Some(path.to_string_lossy().to_string()),
            source_modified_time,
        })
    }

    pub fn import_markdown_folder(&self, path: &Path) -> Result<Vec<Note>, AppError> {
        if !path.is_dir() {
            return Err(AppError::new("notDirectory", "请选择一个文件夹"));
        }

        let category = import_folder_category(path)?;
        let mut files = Vec::new();
        collect_supported_text_files(path, &mut files)?;

        validate_importable_text_files(&files)?;

        self.create_category(&category)?;

        let mut notes = Vec::with_capacity(files.len());
        for file in files {
            notes.push(self.import_markdown_file(&file, &category)?);
        }
        Ok(notes)
    }

    pub fn export_markdown_file(&self, id: &str, path: &Path) -> Result<Note, AppError> {
        self.ensure_storage()?;
        let mut metadata_file = self.load_metadata()?;
        let note = metadata_file
            .notes
            .iter_mut()
            .find(|note| note.id == id)
            .ok_or_else(|| AppError::note_not_found(id))?;
        let content =
            fs::read_to_string(self.note_path_in_category(&note.file_name, &note.category))?;

        self.ensure_export_outside_notes_dir(path)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        write_text_atomic(path, &content)?;

        note.source_path = Some(path.to_string_lossy().to_string());
        note.source_modified_time = Some(file_modified_time_ms(path)?);
        let result = Note {
            id: note.id.clone(),
            title: note.title.clone(),
            file_name: note.file_name.clone(),
            category: note.category.clone(),
            source_path: note.source_path.clone(),
            source_modified_time: note.source_modified_time,
            created_at: note.created_at,
            updated_at: note.updated_at,
            word_count: note.word_count,
            content,
        };

        self.save_metadata(&metadata_file)?;
        Ok(result)
    }

    pub fn list_categories(&self) -> Result<Vec<String>, AppError> {
        let notes_dir = self.notes_dir()?;
        fs::create_dir_all(&notes_dir)?;
        let mut categories = Vec::new();
        for entry in fs::read_dir(&notes_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                categories.push(entry.file_name().to_string_lossy().to_string());
            }
        }
        categories.sort();
        Ok(categories)
    }

    pub fn create_category(&self, name: &str) -> Result<(), AppError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(AppError::category_name_empty());
        }
        validate_category_name(name)?;
        let notes_dir = self.notes_dir()?;
        let path = notes_dir.join(name);
        fs::create_dir_all(&path)?;
        Ok(())
    }

    pub fn rename_category(&self, old_name: &str, new_name: &str) -> Result<(), AppError> {
        let new_name = new_name.trim();
        if new_name.is_empty() {
            return Err(AppError::category_name_empty());
        }
        validate_category_name(old_name)?;
        validate_category_name(new_name)?;
        let notes_dir = self.notes_dir()?;
        let old_path = notes_dir.join(old_name);
        let new_path = notes_dir.join(new_name);
        if !old_path.exists() {
            return Err(AppError::category_not_found(old_name));
        }
        if new_path.exists() {
            return Err(AppError::category_already_exists(new_name));
        }
        fs::rename(&old_path, &new_path)?;

        let mut metadata_file = self.load_metadata()?;
        for note in &mut metadata_file.notes {
            if note.category == old_name {
                note.category = new_name.to_string();
            }
        }
        self.save_metadata(&metadata_file)?;
        Ok(())
    }

    pub fn delete_category(&self, name: &str) -> Result<(), AppError> {
        let notes_dir = self.notes_dir()?;
        let category_path = notes_dir.join(name);
        let dir_exists = category_path.exists();

        if dir_exists {
            // Safety: ensure the category path is actually inside notes_dir
            let canon_notes = fs::canonicalize(&notes_dir).unwrap_or_else(|_| notes_dir.clone());
            let canon_cat =
                fs::canonicalize(&category_path).unwrap_or_else(|_| category_path.clone());
            if !canon_cat.starts_with(&canon_notes) || canon_cat == canon_notes {
                return Err(AppError::new(
                    "unsafePath",
                    format!(
                        "拒绝删除「{}」：路径不在笔记目录内",
                        category_path.display()
                    ),
                ));
            }

            // Move all notes in this category to uncategorized (root)
            let mut metadata_file = self.load_metadata()?;
            for note in &mut metadata_file.notes {
                if note.category == name {
                    let old_path = category_path.join(&note.file_name);
                    let new_path = notes_dir.join(&note.file_name);
                    if old_path.exists() {
                        fs::rename(&old_path, &new_path)?;
                    }
                    note.category = String::new();
                }
            }
            self.save_metadata(&metadata_file)?;

            // Move to recycle bin instead of permanent deletion
            trash::delete(&category_path)
                .map_err(|e| AppError::new("trash", format!("移入回收站失败: {e}")))?;
        } else {
            // Directory already gone (manually deleted outside the app);
            // clean up any stale metadata references.
            let mut metadata_file = self.load_metadata()?;
            let mut changed = false;
            for note in &mut metadata_file.notes {
                if note.category == name {
                    note.category = String::new();
                    changed = true;
                }
            }
            if changed {
                self.save_metadata(&metadata_file)?;
            }
        }
        Ok(())
    }

    pub fn move_note_to_category(
        &self,
        id: &str,
        new_category: &str,
    ) -> Result<NoteMetadata, AppError> {
        self.ensure_storage()?;
        if !new_category.is_empty() {
            validate_category_name(new_category)?;
        }
        let mut metadata_file = self.load_metadata()?;
        let note = metadata_file
            .notes
            .iter_mut()
            .find(|note| note.id == id)
            .ok_or_else(|| AppError::note_not_found(id))?;

        let old_category = note.category.clone();
        if old_category == new_category {
            return Ok(note.clone());
        }

        let old_path = self.note_path_in_category(&note.file_name, &old_category);
        let new_path = self.note_path_in_category(&note.file_name, new_category);
        if let Some(parent) = new_path.parent() {
            fs::create_dir_all(parent)?;
        }
        if old_path.exists() {
            fs::rename(&old_path, &new_path)?;
        }

        note.category = new_category.to_string();
        let result = note.clone();
        self.save_metadata(&metadata_file)?;
        Ok(result)
    }

    pub fn sync_source_file(
        &self,
        id: &str,
        request: SyncSourceRequest,
    ) -> Result<NoteMetadata, AppError> {
        self.ensure_storage()?;
        let mut metadata_file = self.load_metadata()?;
        let note = metadata_file
            .notes
            .iter_mut()
            .find(|note| note.id == id)
            .ok_or_else(|| AppError::note_not_found(id))?;
        let source_path = note
            .source_path
            .clone()
            .filter(|path| !path.trim().is_empty())
            .ok_or_else(|| AppError::new("sourceFileMissing", "该笔记没有关联原文件"))?;
        let source = PathBuf::from(&source_path);
        if !source.is_file() {
            return Err(AppError::source_file_missing(&source_path));
        }

        if !request.force {
            if let Some(expected) = request.expected_modified_time {
                let current = file_modified_time_ms(&source)?;
                if !same_modified_time(current, expected) {
                    return Err(AppError::source_file_conflict(
                        &source_path,
                        current,
                        expected,
                    ));
                }
            }
        }

        write_text_atomic(&source, &request.content)?;
        let updated_modified_time = file_modified_time_ms(&source)?;
        note.source_modified_time = Some(updated_modified_time);
        let result = note.clone();
        self.save_metadata(&metadata_file)?;
        Ok(result)
    }

    pub fn reload_source_file(&self, id: &str) -> Result<Note, AppError> {
        self.ensure_storage()?;
        let mut metadata_file = self.load_metadata()?;
        let note = metadata_file
            .notes
            .iter_mut()
            .find(|note| note.id == id)
            .ok_or_else(|| AppError::note_not_found(id))?;
        let source_path = note
            .source_path
            .clone()
            .filter(|path| !path.trim().is_empty())
            .ok_or_else(|| AppError::new("sourceFileMissing", "该笔记没有关联原文件"))?;
        let source = PathBuf::from(&source_path);
        if !source.is_file() {
            return Err(AppError::source_file_missing(&source_path));
        }

        let content = fs::read_to_string(&source)?;
        let modified_time = file_modified_time_ms(&source)?;
        let now = Utc::now();
        let new_file_name = self.file_name_for(id, &note.title);
        let note_path = self.note_path_in_category(&new_file_name, &note.category);
        let content_changed = match fs::read_to_string(&note_path) {
            Ok(existing) => existing != content,
            Err(_) => true,
        };
        if content_changed {
            if let Some(parent) = note_path.parent() {
                fs::create_dir_all(parent)?;
            }
            write_text_atomic(&note_path, &content)?;
            note.file_name = new_file_name;
            note.updated_at = now;
            note.word_count = count_words(&content);
            note.preview = preview(&content);
        }
        note.source_modified_time = Some(modified_time);

        let result = Note {
            id: note.id.clone(),
            title: note.title.clone(),
            file_name: note.file_name.clone(),
            category: note.category.clone(),
            source_path: note.source_path.clone(),
            source_modified_time: note.source_modified_time,
            created_at: note.created_at,
            updated_at: note.updated_at,
            word_count: note.word_count,
            content,
        };

        self.save_metadata(&metadata_file)?;
        Ok(result)
    }

    fn default_config(&self) -> AppConfig {
        AppConfig {
            locale: default_locale(),
            notes_dir: self.base_dir.join("notes").to_string_lossy().to_string(),
            #[cfg(target_os = "macos")]
            global_shortcut: DEFAULT_MACOS_GLOBAL_SHORTCUT.into(),
            #[cfg(not(target_os = "macos"))]
            global_shortcut: "Ctrl+Space".into(),
            close_to_tray: true,
            autostart: false,
            default_view_mode: "split".into(),
            note_auto_save: true,
            note_surface_auto_save: true,
            tile_color: default_tile_color(),
            tile_color_mode: default_tile_color_mode(),
            theme: default_theme(),
            font_size: default_font_size(),
            surface_font_size: default_surface_font_size(),
            tab_indent_size: default_tab_indent_size(),
            external_file_auto_save: default_external_file_auto_save(),
            background_image_path: String::new(),
            background_fit: default_background_fit(),
            background_dim: default_background_dim(),
            background_blur: default_background_blur(),
            background_scale: default_background_scale(),
            background_position_x: default_background_position(),
            background_position_y: default_background_position(),
            remember_surface_size: default_remember_surface_size(),
            tile_ctrl_close: default_tile_ctrl_close(),
            tile_render_markdown: false,
            render_html_markdown: false,
            surface_width: None,
            surface_height: None,
            toggle_visibility_shortcut: default_toggle_visibility_shortcut(),
            open_at_cursor: default_open_at_cursor(),
        }
    }

    fn ensure_base_dir(&self) -> Result<(), AppError> {
        fs::create_dir_all(&self.base_dir)?;
        Ok(())
    }

    fn ensure_storage(&self) -> Result<(), AppError> {
        self.ensure_base_dir()?;
        let config = self.load_config()?;
        fs::create_dir_all(&config.notes_dir)?;
        if !self.metadata_path().exists() {
            self.save_metadata(&MetadataFile::default())?;
        }
        Ok(())
    }

    fn notes_dir(&self) -> Result<PathBuf, AppError> {
        Ok(PathBuf::from(self.load_config()?.notes_dir))
    }

    fn note_path_in_category(&self, file_name: &str, category: &str) -> PathBuf {
        let notes_dir = self
            .notes_dir()
            .unwrap_or_else(|_| self.base_dir.join("notes"));
        if category.is_empty() {
            notes_dir.join(file_name)
        } else {
            notes_dir.join(category).join(file_name)
        }
    }

    fn ensure_export_outside_notes_dir(&self, path: &Path) -> Result<(), AppError> {
        let Some(parent) = path.parent() else {
            return Ok(());
        };

        let notes_dir = self.notes_dir()?;
        let notes_dir = fs::canonicalize(&notes_dir).unwrap_or(notes_dir);
        let parent = fs::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf());

        if parent.starts_with(&notes_dir) {
            return Err(AppError::new(
                "unsafeExportPath",
                "不能导出到 MiniNote 的内部笔记目录，请选择其他位置",
            ));
        }

        Ok(())
    }

    fn find_metadata(&self, id: &str) -> Result<NoteMetadata, AppError> {
        self.load_metadata()?
            .notes
            .into_iter()
            .find(|note| note.id == id)
            .ok_or_else(|| AppError::note_not_found(id))
    }

    fn file_name_for(&self, id: &str, title: &str) -> String {
        let safe_title = safe_file_stem(title);
        if safe_title.is_empty() {
            format!("{id}.md")
        } else {
            format!("{id}_{safe_title}.md")
        }
    }

    fn load_metadata(&self) -> Result<MetadataFile, AppError> {
        self.ensure_base_dir()?;
        let path = self.metadata_path();
        if !path.exists() {
            let rebuilt = self.rebuild_metadata()?;
            self.save_metadata(&rebuilt)?;
            return Ok(rebuilt);
        }

        match serde_json::from_str(&fs::read_to_string(&path)?) {
            Ok(metadata) => Ok(metadata),
            Err(error) => {
                let corrupt_name = format!(
                    "metadata.corrupt-{}.json",
                    Utc::now().format("%Y%m%d%H%M%S")
                );
                fs::rename(&path, self.base_dir.join(corrupt_name))?;
                let rebuilt = self.rebuild_metadata()?;
                self.save_metadata(&rebuilt)?;
                let _ = error;
                Ok(rebuilt)
            }
        }
    }

    pub(super) fn load_metadata_for_watcher(&self) -> Result<MetadataFile, AppError> {
        self.load_metadata()
    }

    fn save_metadata(&self, metadata: &MetadataFile) -> Result<(), AppError> {
        self.ensure_base_dir()?;
        write_json_atomic(&self.metadata_path(), metadata)
    }

    fn rebuild_metadata(&self) -> Result<MetadataFile, AppError> {
        let notes_dir = self.notes_dir()?;
        fs::create_dir_all(&notes_dir)?;
        let mut notes = Vec::new();

        self.scan_dir_for_notes(&notes_dir, "", &mut notes)?;

        for entry in fs::read_dir(&notes_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let category = entry.file_name().to_string_lossy().to_string();
                self.scan_dir_for_notes(&path, &category, &mut notes)?;
            }
        }

        Ok(MetadataFile { notes })
    }

    fn scan_dir_for_notes(
        &self,
        dir: &Path,
        category: &str,
        notes: &mut Vec<NoteMetadata>,
    ) -> Result<(), AppError> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|extension| extension.to_str()) != Some("md") {
                continue;
            }

            let file_name = entry.file_name().to_string_lossy().to_string();
            let Some(id) = id_from_file_name(&file_name) else {
                continue;
            };
            let content = fs::read_to_string(&path).unwrap_or_default();
            let title = infer_title(&file_name, &content);
            let modified = entry
                .metadata()
                .and_then(|metadata| metadata.modified())
                .map(DateTime::<Utc>::from)
                .unwrap_or_else(|_| Utc::now());

            notes.push(NoteMetadata {
                id,
                title,
                file_name,
                category: category.to_string(),
                source_path: None,
                source_modified_time: None,
                created_at: modified,
                updated_at: modified,
                word_count: count_words(&content),
                preview: preview(&content),
            });
        }
        Ok(())
    }
}

fn safe_file_stem(title: &str) -> String {
    let mut stem = String::new();
    let mut last_was_separator = false;

    for ch in title.trim().chars() {
        let should_separate = ch.is_whitespace()
            || matches!(ch, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*')
            || ch.is_control();

        if should_separate {
            if !stem.is_empty() && !last_was_separator {
                stem.push('_');
                last_was_separator = true;
            }
            continue;
        }

        stem.push(ch);
        last_was_separator = false;
        if stem.chars().count() >= 48 {
            break;
        }
    }

    stem.trim_matches('_').to_string()
}

fn validate_category_name(name: &str) -> Result<(), AppError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(AppError::category_name_empty());
    }
    if name.contains('/') || name.contains('\\') || name.contains(':') || name.contains("..") {
        return Err(AppError::category_name_invalid_chars());
    }
    Ok(())
}

fn normalize_note_category(category: &str) -> Result<String, AppError> {
    let category = category.trim();
    if category.is_empty() {
        return Ok(String::new());
    }
    validate_category_name(category)?;
    Ok(category.to_string())
}

pub(super) fn file_modified_time_ms(path: &Path) -> Result<f64, AppError> {
    let modified = fs::metadata(path)?.modified()?;
    let duration = modified
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    Ok(duration.as_secs_f64() * 1000.0)
}

pub(super) fn same_modified_time(left: f64, right: f64) -> bool {
    (left - right).abs() < 1.0
}

fn count_words(content: &str) -> usize {
    content.chars().filter(|ch| !ch.is_whitespace()).count()
}

fn preview(content: &str) -> String {
    content
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(80)
        .collect()
}

fn id_from_file_name(file_name: &str) -> Option<String> {
    let stem = file_name.strip_suffix(".md")?;
    Some(
        stem.split_once('_')
            .map(|(id, _)| id.to_string())
            .unwrap_or_else(|| stem.to_string()),
    )
}

fn infer_title(file_name: &str, content: &str) -> String {
    if let Some(title) = content
        .lines()
        .find_map(|line| line.trim().strip_prefix("# ").map(str::trim))
        .filter(|title| !title.is_empty())
    {
        return title.to_string();
    }

    let stem = file_name.strip_suffix(".md").unwrap_or(file_name);
    stem.split_once('_')
        .map(|(_, title)| title.replace('_', " "))
        .unwrap_or_default()
}

fn is_supported_text_path(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "mint" | "md" | "markdown" | "txt"
            )
        })
        .unwrap_or(false)
}

fn collect_supported_text_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), AppError> {
    let mut entries = fs::read_dir(dir)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            collect_supported_text_files(&path, files)?;
        } else if file_type.is_file() && is_supported_text_path(&path) {
            files.push(path);
        }
    }

    Ok(())
}

fn import_folder_category(path: &Path) -> Result<String, AppError> {
    let category = path
        .file_name()
        .map(|name| name.to_string_lossy().trim().to_string())
        .filter(|name| !name.is_empty())
        .ok_or_else(AppError::category_name_empty)?;
    validate_category_name(&category)?;
    Ok(category)
}

fn validate_importable_text_files(files: &[PathBuf]) -> Result<(), AppError> {
    for file in files {
        let file_size = fs::metadata(file)?.len();
        if file_size > MAX_IMPORT_TEXT_SIZE {
            return Err(AppError::file_too_large(MAX_IMPORT_TEXT_SIZE / 1024 / 1024));
        }
        let _ = fs::read_to_string(file)?;
    }
    Ok(())
}

fn imported_markdown_title(path: &Path, content: &str) -> String {
    let first_line = content.lines().next().unwrap_or_default();
    let first_line = first_line.trim_start_matches('\u{feff}').trim_start();

    if let Some(title) = first_line
        .strip_prefix("# ")
        .map(str::trim)
        .filter(|title| !title.is_empty())
    {
        return title.to_string();
    }

    path.file_stem()
        .and_then(|file_stem| file_stem.to_str())
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .unwrap_or("导入笔记")
        .to_string()
}

fn default_note_auto_save() -> bool {
    true
}

fn default_note_surface_auto_save() -> bool {
    true
}

fn default_tile_color() -> String {
    "#f6f3ec".into()
}

fn default_tile_color_mode() -> String {
    "system".into()
}

fn default_theme() -> String {
    "system".into()
}

fn default_font_size() -> u32 {
    14
}

fn default_surface_font_size() -> u32 {
    14
}

fn default_tab_indent_size() -> u32 {
    2
}

fn default_external_file_auto_save() -> bool {
    true
}

fn default_background_fit() -> String {
    "cover".into()
}

fn default_background_dim() -> f64 {
    0.25
}

fn default_background_blur() -> f64 {
    0.0
}

fn default_background_scale() -> f64 {
    1.0
}

fn default_background_position() -> f64 {
    50.0
}

fn default_remember_surface_size() -> bool {
    true
}

fn default_tile_ctrl_close() -> bool {
    true
}

fn default_toggle_visibility_shortcut() -> String {
    String::new()
}

fn default_open_at_cursor() -> bool {
    true
}

fn default_locale() -> String {
    "zh-CN".into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    fn test_root(name: &str) -> PathBuf {
        let base = std::env::var_os("MININOTE_TEST_TEMP_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| std::env::temp_dir().join("mininote-rust-tests"));
        let root = base.join(name);
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove stale test root");
        }
        fs::create_dir_all(&root).expect("create test root");
        root
    }

    fn request(title: &str, content: &str, category: &str) -> SaveNoteRequest {
        SaveNoteRequest {
            title: title.into(),
            content: content.into(),
            category: category.into(),
            source_path: None,
            source_modified_time: None,
        }
    }

    #[test]
    fn creates_updates_reads_and_deletes_markdown_notes() {
        let store = NoteStore::new(test_root("crud"));

        let created = store
            .create_note(request("A/B:Test", "hello\nworld", ""))
            .expect("create note");

        assert_eq!(created.title, "A/B:Test");
        assert_eq!(created.content, "hello\nworld");
        assert_eq!(created.word_count, 10);
        assert!(created.file_name.ends_with(".md"));
        assert!(created.file_name.contains("A_B_Test"));

        let loaded = store.read_note(&created.id).expect("read note");
        assert_eq!(loaded, created);

        let listed = store.list_notes().expect("list notes");
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, created.id);
        assert_eq!(listed[0].preview, "hello world");

        let updated = store
            .update_note(&created.id, request("", "# 新标题\nsecond line", ""))
            .expect("update note");

        assert_eq!(updated.title, "");
        assert_eq!(updated.content, "# 新标题\nsecond line");
        assert_ne!(updated.file_name, created.file_name);

        store.delete_note(&created.id).expect("delete note");
        assert!(store.read_note(&created.id).is_err());
        assert!(store.list_notes().expect("list after delete").is_empty());
    }

    #[test]
    fn rebuilds_metadata_when_metadata_json_is_corrupt() {
        let store = NoteStore::new(test_root("repair"));
        let first = store
            .create_note(request("第一条", "# 第一条\n正文", ""))
            .expect("create first");
        let second = store
            .create_note(request("第二条", "第二条正文", ""))
            .expect("create second");

        fs::write(store.metadata_path(), "{ broken json").expect("corrupt metadata");

        let repaired = store.list_notes().expect("repair metadata");
        let ids: Vec<_> = repaired.iter().map(|note| note.id.as_str()).collect();

        assert_eq!(repaired.len(), 2);
        assert!(ids.contains(&first.id.as_str()));
        assert!(ids.contains(&second.id.as_str()));
        assert!(store
            .base_dir()
            .read_dir()
            .expect("read base dir")
            .any(|entry| entry
                .expect("entry")
                .file_name()
                .to_string_lossy()
                .starts_with("metadata.corrupt-")));
    }

    #[test]
    fn reads_and_writes_config_json() {
        let store = NoteStore::new(test_root("config"));

        let default_config = store.load_config().expect("load default config");
        #[cfg(target_os = "macos")]
        assert_eq!(default_config.global_shortcut, "Command+Option+N");
        #[cfg(not(target_os = "macos"))]
        assert_eq!(default_config.global_shortcut, "Ctrl+Space");
        assert!(default_config.note_auto_save);
        assert!(default_config.note_surface_auto_save);
        assert_eq!(default_config.tile_color, "#f6f3ec");
        assert_eq!(default_config.tile_color_mode, "system");
        assert_eq!(default_config.theme, "system");
        assert_eq!(default_config.locale, "zh-CN");
        assert!(default_config.notes_dir.ends_with("notes"));

        let custom_notes_dir = store.base_dir().join("custom-notes");
        let saved = AppConfig {
            locale: "en-US".into(),
            notes_dir: custom_notes_dir.join("notes").to_string_lossy().to_string(),
            global_shortcut: "Alt+Space".into(),
            close_to_tray: false,
            autostart: true,
            default_view_mode: "preview".into(),
            note_auto_save: false,
            note_surface_auto_save: false,
            tile_color: "#efe8dc".into(),
            tile_color_mode: "custom".into(),
            theme: "dark".into(),
            font_size: 16,
            surface_font_size: 16,
            tab_indent_size: 2,
            external_file_auto_save: true,
            background_image_path: String::new(),
            background_fit: "cover".into(),
            background_dim: 0.25,
            background_blur: 0.0,
            background_scale: 1.0,
            background_position_x: 50.0,
            background_position_y: 50.0,
            remember_surface_size: true,
            tile_ctrl_close: true,
            tile_render_markdown: false,
            render_html_markdown: false,
            surface_width: None,
            surface_height: None,
            toggle_visibility_shortcut: String::new(),
            open_at_cursor: true,
        };

        store.save_config(saved.clone()).expect("save config");

        let loaded = store.load_config().expect("reload config");
        assert_eq!(loaded, saved);
        assert!(custom_notes_dir.exists());
    }

    #[test]
    fn loads_legacy_config_with_note_surface_auto_save_enabled() {
        let store = NoteStore::new(test_root("legacy-config"));
        let notes_dir = store.base_dir().join("notes");
        fs::create_dir_all(&notes_dir).expect("create notes dir");
        fs::write(
            store.config_path(),
            format!(
                r#"{{
  "notesDir": "{}",
  "globalShortcut": "Ctrl+Space",
  "closeToTray": true,
  "autostart": false,
  "defaultViewMode": "split"
}}"#,
                notes_dir.to_string_lossy().replace('\\', "\\\\")
            ),
        )
        .expect("write legacy config");

        let loaded = store.load_config().expect("load legacy config");

        assert!(loaded.note_auto_save);
        assert!(loaded.note_surface_auto_save);
        assert_eq!(loaded.tile_color, "#f6f3ec");
        assert_eq!(loaded.tile_color_mode, "system");
        assert_eq!(loaded.theme, "system");
        assert_eq!(loaded.locale, "zh-CN");
        assert_eq!(loaded.font_size, 14);
        assert_eq!(loaded.surface_font_size, 14);
    }

    #[test]
    fn imports_markdown_heading_title_without_stripping_content() {
        let root = test_root("import-heading-title");
        let source_path = root.join("外部文件.md");
        let source_content = "# 导入标题\n正文第一行\n正文第二行";
        fs::write(&source_path, source_content).expect("write source markdown");
        let store = NoteStore::new(root.join("store"));

        let imported = store
            .import_markdown_file(&source_path, "")
            .expect("import markdown");

        assert_eq!(imported.title, "导入标题");
        assert_eq!(imported.content, source_content);
        assert_eq!(
            imported.source_path.as_deref(),
            Some(source_path.to_string_lossy().as_ref())
        );
        assert!(imported.source_modified_time.is_some());
        assert_eq!(
            store
                .read_note(&imported.id)
                .expect("read imported")
                .content,
            source_content
        );
    }

    #[test]
    fn imports_markdown_title_from_file_name_without_heading() {
        let root = test_root("import-file-title");
        let source_path = root.join("会议记录.md");
        let source_content = "正文第一行\n# 不是第一行标题";
        fs::write(&source_path, source_content).expect("write source markdown");
        let store = NoteStore::new(root.join("store"));

        let imported = store
            .import_markdown_file(&source_path, "")
            .expect("import markdown");

        assert_eq!(imported.title, "会议记录");
        assert_eq!(imported.content, source_content);
    }

    #[test]
    fn imports_supported_text_files_from_folder_into_matching_category() {
        let root = test_root("import-folder");
        let source_dir = root.join("项目资料");
        let nested_dir = source_dir.join("nested");
        fs::create_dir_all(&nested_dir).expect("create source dirs");
        fs::write(source_dir.join("a.md"), "# A\n正文").expect("write markdown");
        fs::write(source_dir.join("b.mint"), "Mint 正文").expect("write mint");
        fs::write(nested_dir.join("c.txt"), "Text 正文").expect("write text");
        fs::write(nested_dir.join("ignored.png"), "image").expect("write ignored");
        let store = NoteStore::new(root.join("store"));

        let imported = store
            .import_markdown_folder(&source_dir)
            .expect("import folder");

        assert_eq!(imported.len(), 3);
        assert!(imported.iter().all(|note| note.category == "项目资料"));
        assert_eq!(
            store.list_categories().expect("list categories"),
            vec!["项目资料"]
        );
        assert_eq!(store.list_notes().expect("list notes").len(), 3);

        let mut titles = imported
            .iter()
            .map(|note| note.title.as_str())
            .collect::<Vec<_>>();
        titles.sort();
        assert_eq!(titles, vec!["A", "b", "c"]);

        let source_paths = imported
            .iter()
            .filter_map(|note| note.source_path.as_deref())
            .collect::<Vec<_>>();
        assert_eq!(source_paths.len(), 3);
        assert!(source_paths.iter().any(|path| path.ends_with("a.md")));
        assert!(source_paths.iter().any(|path| path.ends_with("b.mint")));
        assert!(source_paths.iter().any(|path| path.ends_with("c.txt")));
    }

    #[test]
    fn import_folder_creates_empty_category_when_no_supported_text_files() {
        let root = test_root("import-empty-folder");
        let source_dir = root.join("空分类");
        fs::create_dir_all(&source_dir).expect("create source dir");
        fs::write(source_dir.join("ignored.png"), "image").expect("write ignored");
        let store = NoteStore::new(root.join("store"));

        let imported = store
            .import_markdown_folder(&source_dir)
            .expect("import empty folder");

        assert!(imported.is_empty());
        assert_eq!(
            store.list_categories().expect("list categories"),
            vec!["空分类"]
        );
    }

    #[test]
    fn import_folder_preflights_all_files_before_creating_notes() {
        let root = test_root("import-folder-preflight");
        let source_dir = root.join("项目资料");
        fs::create_dir_all(&source_dir).expect("create source dir");
        fs::write(source_dir.join("a.md"), "# A\n正文").expect("write markdown");
        fs::write(source_dir.join("broken.txt"), [0xff, 0xfe]).expect("write invalid text");
        let store = NoteStore::new(root.join("store"));

        store
            .import_markdown_folder(&source_dir)
            .expect_err("invalid text should fail folder import");

        assert!(store.list_categories().expect("list categories").is_empty());
        assert!(store.list_notes().expect("list notes").is_empty());
    }

    #[test]
    fn rejects_oversized_imported_text_files() {
        let root = test_root("import-file-too-large");
        let source_path = root.join("large.md");
        let file = fs::File::create(&source_path).expect("create large source");
        file.set_len(MAX_IMPORT_TEXT_SIZE + 1)
            .expect("size large source");
        let store = NoteStore::new(root.join("store"));

        let error = store
            .import_markdown_file(&source_path, "")
            .expect_err("oversized text import should fail");

        assert_eq!(error.code, "fileTooLarge");
        assert_eq!(error.details.get("maxMb").map(String::as_str), Some("10"));
    }

    #[test]
    fn exports_markdown_file_without_rewriting_content() {
        let root = test_root("export-markdown");
        let store = NoteStore::new(root.join("store"));
        let content = "# 原始标题\n正文\n- 列表";
        let note = store
            .create_note(request("导出标题", content, ""))
            .expect("create note");
        let export_path = root.join("exports").join("导出.md");

        let exported = store
            .export_markdown_file(&note.id, &export_path)
            .expect("export markdown");

        assert_eq!(
            fs::read_to_string(export_path).expect("read exported markdown"),
            content
        );
        let expected_source_path = root
            .join("exports")
            .join("导出.md")
            .to_string_lossy()
            .to_string();
        assert_eq!(
            exported.source_path.as_deref(),
            Some(expected_source_path.as_str())
        );
        assert!(exported.source_modified_time.is_some());

        store
            .sync_source_file(
                &note.id,
                SyncSourceRequest {
                    content: "导出后继续编辑".into(),
                    expected_modified_time: exported.source_modified_time,
                    force: false,
                },
            )
            .expect("sync exported source");
        assert_eq!(
            fs::read_to_string(root.join("exports").join("导出.md"))
                .expect("read synced exported source"),
            "导出后继续编辑"
        );
    }

    #[test]
    fn rejects_exporting_into_internal_notes_storage() {
        let root = test_root("export-internal-notes-dir");
        let store = NoteStore::new(root.join("store"));
        let note = store
            .create_note(request("内部路径", "正文", ""))
            .expect("create note");
        let internal_path = store.note_path_in_category(&note.file_name, &note.category);

        let error = store
            .export_markdown_file(&note.id, &internal_path)
            .expect_err("exporting into internal notes dir should fail");

        assert_eq!(error.code, "unsafeExportPath");
        assert_eq!(
            store
                .read_note(&note.id)
                .expect("read note after failed export")
                .source_path,
            None
        );
    }

    #[test]
    fn syncs_source_file_and_detects_conflicts() {
        let root = test_root("source-sync");
        let source_path = root.join("source.md");
        fs::write(&source_path, "original").expect("write source");
        let store = NoteStore::new(root.join("store"));
        let imported = store
            .import_markdown_file(&source_path, "")
            .expect("import source");

        let synced = store
            .sync_source_file(
                &imported.id,
                SyncSourceRequest {
                    content: "from mininote".into(),
                    expected_modified_time: imported.source_modified_time,
                    force: false,
                },
            )
            .expect("sync source");

        assert_eq!(
            fs::read_to_string(&source_path).expect("read synced source"),
            "from mininote"
        );
        assert!(
            !source_path.with_file_name("source.md.tmp").exists(),
            "source sync should not leave a temporary file"
        );
        assert!(synced.source_modified_time.is_some());

        fs::write(&source_path, "changed elsewhere").expect("external edit");
        let error = store
            .sync_source_file(
                &imported.id,
                SyncSourceRequest {
                    content: "new local content".into(),
                    expected_modified_time: synced.source_modified_time,
                    force: false,
                },
            )
            .expect_err("changed source should conflict");
        assert_eq!(error.code, "sourceFileConflict");

        store
            .sync_source_file(
                &imported.id,
                SyncSourceRequest {
                    content: "forced".into(),
                    expected_modified_time: synced.source_modified_time,
                    force: true,
                },
            )
            .expect("force sync");
        assert_eq!(
            fs::read_to_string(&source_path).expect("read forced source"),
            "forced"
        );
    }

    #[test]
    fn rejects_invalid_category_names_for_create_update_and_import() {
        let root = test_root("category-validation-write-entrypoints");
        let source_path = root.join("source.md");
        fs::write(&source_path, "source").expect("write source");
        let store = NoteStore::new(root.join("store"));

        let create_error = store
            .create_note(request("标题", "正文", "../escape"))
            .expect_err("invalid create category should fail");
        assert_eq!(create_error.code, "categoryNameInvalidChars");

        let note = store
            .create_note(request("标题", "正文", ""))
            .expect("create note");
        let update_error = store
            .update_note(&note.id, request("标题", "正文", "safe/../escape"))
            .expect_err("invalid update category should fail");
        assert_eq!(update_error.code, "categoryNameInvalidChars");

        let import_error = store
            .import_markdown_file(&source_path, "../escape")
            .expect_err("invalid import category should fail");
        assert_eq!(import_error.code, "categoryNameInvalidChars");
    }

    #[test]
    fn update_note_preserves_existing_source_link() {
        let root = test_root("source-link-preserve");
        let source_path = root.join("source.md");
        let other_path = root.join("other.md");
        fs::write(&source_path, "original").expect("write source");
        fs::write(&other_path, "other").expect("write other");
        let store = NoteStore::new(root.join("store"));
        let imported = store
            .import_markdown_file(&source_path, "")
            .expect("import source");

        let updated = store
            .update_note(
                &imported.id,
                SaveNoteRequest {
                    title: "updated".into(),
                    content: "local".into(),
                    category: String::new(),
                    source_path: Some(other_path.to_string_lossy().to_string()),
                    source_modified_time: Some(1.0),
                },
            )
            .expect("update note");

        assert_eq!(
            updated.source_path.as_deref(),
            Some(source_path.to_string_lossy().as_ref())
        );
        assert_ne!(updated.source_modified_time, Some(1.0));
        assert!(same_modified_time(
            updated.source_modified_time.expect("updated source mtime"),
            imported
                .source_modified_time
                .expect("imported source mtime")
        ));
    }

    #[test]
    fn reloads_only_the_recorded_source_file() {
        let root = test_root("source-reload");
        let source_path = root.join("source.md");
        fs::write(&source_path, "disk version").expect("write source");
        let store = NoteStore::new(root.join("store"));
        let imported = store
            .import_markdown_file(&source_path, "")
            .expect("import source");

        fs::write(&source_path, "new disk version").expect("update source");
        let reloaded = store
            .reload_source_file(&imported.id)
            .expect("reload source");

        assert_eq!(reloaded.content, "new disk version");
        assert_eq!(reloaded.source_path, imported.source_path);
        assert!(reloaded.source_modified_time.is_some());
        assert_eq!(
            store
                .read_note(&imported.id)
                .expect("read reloaded")
                .content,
            "new disk version"
        );
    }

    #[test]
    fn rejects_svg_image_saves() {
        let store = NoteStore::new(test_root("image-format"));
        let note = store
            .create_note(request("标题", "正文", ""))
            .expect("create note");
        let error = store
            .save_image(&note.id, b"<svg></svg>", "svg")
            .expect_err("svg image should be rejected");

        assert_eq!(error.code, "unsupportedImageFormat");
    }

    #[test]
    fn rejects_oversized_image_saves() {
        let store = NoteStore::new(test_root("image-too-large"));
        let note = store
            .create_note(request("标题", "正文", ""))
            .expect("create note");
        let data = vec![0; MAX_IMAGE_SIZE + 1];

        let error = store
            .save_image(&note.id, &data, "png")
            .expect_err("oversized image should be rejected");

        assert_eq!(error.code, "imageTooLarge");
        assert_eq!(error.details.get("maxMb").map(String::as_str), Some("30"));
    }

    #[test]
    fn rejects_invalid_category_names_for_rename_and_move() {
        let root = test_root("category-validation");
        let store = NoteStore::new(root);
        store.create_category("工作").expect("create category");
        let note = store
            .create_note(request("标题", "正文", "工作"))
            .expect("create note");

        let rename_error = store
            .rename_category("../工作", "新分类")
            .expect_err("invalid old category should fail");
        assert_eq!(rename_error.code, "categoryNameInvalidChars");

        let move_error = store
            .move_note_to_category(&note.id, "../逃逸")
            .expect_err("invalid target category should fail");
        assert_eq!(move_error.code, "categoryNameInvalidChars");
    }
}
