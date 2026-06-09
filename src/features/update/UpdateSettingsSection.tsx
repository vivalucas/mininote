import { useEffect, useRef, useState } from "react";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { openUrl } from "@tauri-apps/plugin-opener";
import { useTranslation } from "react-i18next";
import { checkForUpdates, getUpdateStatus } from "./api";
import {
  getInitialUpdateStatusNotice,
  getUpdateCheckCompletionNotice,
  type UpdateInlineNotice,
} from "./presentation";
import { getUpdateErrorMessage } from "./updateErrors";
import type { UpdateErrorPayload, UpdateState } from "./types";

const RELEASES_URL = "https://github.com/vivalucas/mininote/releases/latest";

type BusyAction = "checking" | null;

interface UpdateSettingsSectionProps {
  initialStatus?: UpdateState;
}

export function UpdateSettingsSection({ initialStatus }: UpdateSettingsSectionProps) {
  const { t } = useTranslation();
  const [status, setStatus] = useState<UpdateState | null>(initialStatus ?? null);
  const [busyAction, setBusyAction] = useState<BusyAction>(null);
  const [notice, setNotice] = useState<UpdateInlineNotice | null>(() =>
    getInitialUpdateStatusNotice(initialStatus, t),
  );
  const translateRef = useRef(t);

  useEffect(() => {
    translateRef.current = t;
  }, [t]);

  useEffect(() => {
    if (initialStatus) return;
    let alive = true;

    getUpdateStatus()
      .then((loadedStatus) => {
        if (!alive) return;
        setStatus(loadedStatus);
        setNotice(getInitialUpdateStatusNotice(loadedStatus, t));
      })
      .catch((error) => {
        if (!alive) return;
        setNotice({ tone: "error", text: getUpdateErrorMessage(error, t) });
      });

    return () => {
      alive = false;
    };
  }, [initialStatus, t]);

  useEffect(() => {
    let active = true;

    const bindEvents = async () => {
      const unlistenFns: UnlistenFn[] = [];
      const disposeAll = () => {
        for (const unlisten of unlistenFns.splice(0)) {
          unlisten();
        }
      };

      try {
        unlistenFns.push(
          await listen<UpdateState>("update://checking", (event) => {
            if (!active) return;
            setStatus(event.payload);
          }),
        );

        unlistenFns.push(
          await listen<UpdateState>("update://checked", (event) => {
            if (!active) return;
            setStatus(event.payload);
            const nextNotice = getUpdateCheckCompletionNotice(event.payload, translateRef.current);
            if (nextNotice) {
              setNotice(nextNotice);
            }
          }),
        );

        unlistenFns.push(
          await listen<UpdateErrorPayload>("update://error", (event) => {
            if (!active) return;
            setNotice({
              tone: "error",
              text: getUpdateErrorMessage(event.payload, translateRef.current),
            });
          }),
        );

        unlistenFns.push(
          await listen<UpdateErrorPayload>("update://auto-check-error", (event) => {
            if (!active) return;
            setNotice({
              tone: "error",
              text: getUpdateErrorMessage(event.payload, translateRef.current),
            });
          }),
        );

        return disposeAll;
      } catch (error) {
        disposeAll();
        console.error("failed to bind update settings event listeners", error);
        return () => undefined;
      }
    };

    const promise = bindEvents();

    return () => {
      active = false;
      void promise
        .then((dispose) => dispose())
        .catch((error) => {
          console.error("failed to dispose update settings event listeners", error);
        });
    };
  }, []);

  const currentVersion = status?.currentVersion ?? "--";
  const hasUpdate = status?.status === "available" && !!status.latestVersion;
  const controlsDisabled = busyAction !== null || status?.status === "checking";

  const handleCheck = async () => {
    setBusyAction("checking");
    setNotice(null);
    try {
      const result = await checkForUpdates(true);
      setNotice({
        tone: "success",
        text:
          result.status === "available"
            ? t("settings.update.available", {
                version: result.latestVersion,
                defaultValue: "发现新版本 {{version}}",
              })
            : t("settings.update.notAvailable", {
                defaultValue: "当前已是最新版本",
              }),
      });
    } catch (error) {
      setNotice({ tone: "error", text: getUpdateErrorMessage(error, t) });
    } finally {
      try {
        setStatus(await getUpdateStatus());
      } catch (refreshError) {
        console.warn("Failed to refresh update status after check", refreshError);
      }
      setBusyAction(null);
    }
  };

  return (
    <section className="space-y-3 pt-2 border-t border-paper-deep/25">
      <div className="flex items-center justify-between gap-2">
        <div>
          <h3 className="text-[11px] font-body text-ink-faint">
            {t("settings.update.title", { defaultValue: "更新" })}
          </h3>
          <p className="mt-1 text-[10px] font-mono text-ink-ghost">
            {busyAction === "checking" || status?.status === "checking"
              ? t("settings.update.checking", {
                  defaultValue: "正在检查...",
                })
              : notice
                ? notice.text
                : t("settings.update.currentVersion", {
                    version: currentVersion,
                    defaultValue: "当前版本：{{version}}",
                  })}
          </p>
        </div>
        <button
          type="button"
          disabled={controlsDisabled}
          onClick={() => void handleCheck()}
          className="h-8 px-3 rounded-lg border border-paper-deep/45 text-[11px] text-ink-faint hover:text-bamboo hover:bg-bamboo-mist/50 disabled:opacity-50 disabled:cursor-not-allowed transition-colors cursor-pointer"
        >
          {busyAction === "checking"
            ? t("settings.update.busy", { defaultValue: "处理中" })
            : t("settings.update.check", { defaultValue: "检查更新" })}
        </button>
      </div>

      {hasUpdate ? (
        <div className="space-y-2 rounded-lg border border-paper-deep/25 bg-paper-warm/40 px-3 py-3">
          <div className="space-y-1">
            <p className="text-[11px] font-body text-ink-faint">
              {t("settings.update.latestVersion", {
                version: status.latestVersion,
                defaultValue: "待更新版本：{{version}}",
              })}
            </p>
            <p className="text-[10px] text-ink-ghost">
              {t("settings.update.manualDownloadHint", {
                defaultValue: "请前往 GitHub Releases 下载并安装最新版本。",
              })}
            </p>
          </div>
          <button
            type="button"
            onClick={() => void openUrl(RELEASES_URL)}
            className="h-8 px-3 rounded-lg bg-bamboo text-[11px] text-paper hover:bg-bamboo-light transition-colors cursor-pointer"
          >
            {t("settings.update.openRelease", { defaultValue: "打开 Release 页面" })}
          </button>
        </div>
      ) : null}
    </section>
  );
}
