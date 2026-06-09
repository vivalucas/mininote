pub mod desktop;
pub mod json_io;
pub mod locales;
pub mod services;
pub mod updater;

use locales::Locale;
use services::notes::{
    default_store, AppConfig, AppError, Note, NoteMetadata, SaveNoteRequest, SyncSourceRequest,
};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Emitter, Manager};

const MAX_BACKGROUND_IMAGE_SIZE: u64 = 30 * 1024 * 1024;
const ALLOWED_BACKGROUND_IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp"];

#[tauri::command]
fn app_name() -> Result<String, AppError> {
    let locale = Locale::from_tag(&default_store()?.load_config()?.locale);
    Ok(locales::app_name(locale).to_string())
}

#[tauri::command]
fn notes_list() -> Result<Vec<NoteMetadata>, AppError> {
    default_store()?.list_notes()
}

#[tauri::command]
fn notes_get(id: String) -> Result<Note, AppError> {
    default_store()?.read_note(&id)
}

#[tauri::command]
fn notes_create(app: AppHandle, request: SaveNoteRequest) -> Result<Note, AppError> {
    let note = default_store()?.create_note(strip_source_link(request))?;
    let _ = app.emit("notes-changed", ());
    Ok(note)
}

#[tauri::command]
fn notes_update(app: AppHandle, id: String, request: SaveNoteRequest) -> Result<Note, AppError> {
    let note = default_store()?.update_note(&id, strip_source_link(request))?;
    let _ = app.emit("notes-changed", ());
    Ok(note)
}

#[tauri::command]
fn notes_delete(app: AppHandle, id: String) -> Result<(), AppError> {
    default_store()?.delete_note(&id)?;
    let _ = app.emit("notes-changed", ());
    Ok(())
}

#[tauri::command]
fn notes_import_markdown(
    app: AppHandle,
    path: String,
    category: Option<String>,
) -> Result<Note, AppError> {
    let note = default_store()?
        .import_markdown_file(&PathBuf::from(path), &category.unwrap_or_default())?;
    let _ = app.emit("notes-changed", ());
    Ok(note)
}

#[tauri::command]
fn notes_export_markdown(id: String, path: String) -> Result<(), AppError> {
    default_store()?.export_markdown_file(&id, &PathBuf::from(path))
}

#[tauri::command]
fn notes_sync_source_file(
    app: AppHandle,
    id: String,
    request: SyncSourceRequest,
) -> Result<NoteMetadata, AppError> {
    let metadata = default_store()?.sync_source_file(&id, request)?;
    let _ = app.emit("notes-changed", ());
    Ok(metadata)
}

#[tauri::command]
fn notes_reload_source_file(app: AppHandle, id: String) -> Result<Note, AppError> {
    let note = default_store()?.reload_source_file(&id)?;
    let _ = app.emit("notes-changed", ());
    Ok(note)
}

fn strip_source_link(mut request: SaveNoteRequest) -> SaveNoteRequest {
    request.source_path = None;
    request.source_modified_time = None;
    request
}

#[tauri::command]
fn categories_list() -> Result<Vec<String>, AppError> {
    default_store()?.list_categories()
}

#[tauri::command]
fn categories_create(app: AppHandle, name: String) -> Result<(), AppError> {
    default_store()?.create_category(&name)?;
    let _ = app.emit("notes-changed", ());
    Ok(())
}

#[tauri::command]
fn categories_rename(app: AppHandle, old_name: String, new_name: String) -> Result<(), AppError> {
    default_store()?.rename_category(&old_name, &new_name)?;
    let _ = app.emit("notes-changed", ());
    Ok(())
}

#[tauri::command]
fn categories_delete(app: AppHandle, name: String) -> Result<(), AppError> {
    default_store()?.delete_category(&name)?;
    let _ = app.emit("notes-changed", ());
    Ok(())
}

#[tauri::command]
fn notes_move_category(
    app: AppHandle,
    id: String,
    category: String,
) -> Result<NoteMetadata, AppError> {
    let result = default_store()?.move_note_to_category(&id, &category)?;
    let _ = app.emit("notes-changed", ());
    Ok(result)
}

#[tauri::command]
fn images_save(note_id: String, data: Vec<u8>, extension: String) -> Result<String, AppError> {
    default_store()?.save_image(&note_id, &data, &extension)
}

#[tauri::command]
fn images_get_base_dir() -> Result<String, AppError> {
    let store = default_store()?;
    store
        .base_dir()
        .to_str()
        .map(str::to_string)
        .ok_or_else(|| AppError {
            code: "path".into(),
            message: "invalid base dir path".into(),
            details: Default::default(),
        })
}

#[tauri::command]
fn images_clean_unused(note_id: String, content: String) -> Result<Vec<String>, AppError> {
    default_store()?.clean_unused_images(&note_id, &content)
}

#[tauri::command]
fn config_get() -> Result<AppConfig, AppError> {
    default_store()?.load_config()
}

#[tauri::command]
fn copy_background_image(_app: AppHandle, source_path: String) -> Result<String, AppError> {
    let source = PathBuf::from(source_path.trim());
    if !source.is_file() {
        return Err(AppError {
            code: "invalidSource".into(),
            message: "background image source not found".into(),
            details: Default::default(),
        });
    }
    let ext = source
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .ok_or_else(|| AppError {
            code: "unsupportedImageFormat".into(),
            message: "unsupported background image format".into(),
            details: Default::default(),
        })?;
    if !ALLOWED_BACKGROUND_IMAGE_EXTENSIONS.contains(&ext.as_str()) {
        return Err(AppError {
            code: "unsupportedImageFormat".into(),
            message: "unsupported background image format".into(),
            details: Default::default(),
        });
    }
    let source_size = fs::metadata(&source)?.len();
    if source_size > MAX_BACKGROUND_IMAGE_SIZE {
        return Err(AppError {
            code: "imageTooLarge".into(),
            message: "background image is too large".into(),
            details: Default::default(),
        });
    }

    let store = default_store()?;
    let dir = store.base_dir().join("backgrounds");
    fs::create_dir_all(&dir)?;

    let old_config = store.load_config()?;
    if !old_config.background_image_path.is_empty() {
        let old_path = PathBuf::from(&old_config.background_image_path);
        if old_path.starts_with(&dir) && old_path.is_file() {
            let _ = fs::remove_file(&old_path);
        }
    }

    let dest = dir.join(format!("bg-{}.{}", uuid::Uuid::new_v4(), ext));
    fs::copy(&source, &dest)?;

    dest.to_str().map(str::to_string).ok_or_else(|| AppError {
        code: "path".into(),
        message: "invalid destination path".into(),
        details: Default::default(),
    })
}

#[tauri::command]
fn config_save(app: AppHandle, config: AppConfig) -> Result<AppConfig, AppError> {
    let store = default_store()?;
    let previous = store.load_config()?;
    desktop::apply_runtime_config(&app, &previous, &config).map_err(|error| {
        match error.downcast::<AppError>() {
            Ok(app_error) => *app_error,
            Err(error) => AppError {
                code: "desktopConfig".into(),
                message: error.to_string(),
                details: Default::default(),
            },
        }
    })?;
    let saved = store.save_config(config)?;
    if let Err(error) = desktop::refresh_shell_state(&app, &saved) {
        eprintln!("failed to refresh desktop shell state: {error}");
    }
    let _ = app.emit("config-changed", &saved);
    Ok(saved)
}

#[tauri::command]
fn global_shortcut_check(
    app: AppHandle,
    shortcut: String,
) -> Result<desktop::ShortcutCheckResult, AppError> {
    desktop::check_global_shortcut(&app, &shortcut)
}

#[tauri::command]
fn start_shortcut_recording(app: AppHandle) -> Result<(), AppError> {
    desktop::start_shortcut_recording(&app).map_err(|error| AppError {
        code: "shortcutRecording".into(),
        message: error.to_string(),
        details: Default::default(),
    })
}

#[tauri::command]
fn stop_shortcut_recording(app: AppHandle) -> Result<(), AppError> {
    desktop::stop_shortcut_recording(&app).map_err(|error| AppError {
        code: "shortcutRecording".into(),
        message: error.to_string(),
        details: Default::default(),
    })
}

#[tauri::command]
async fn open_notepad_window(
    app: AppHandle,
    note_id: Option<String>,
    bounds: Option<desktop::WindowBounds>,
) -> Result<String, AppError> {
    desktop::open_notepad_window(app, note_id, bounds).await
}

#[tauri::command]
async fn recycle_notepad_window(app: AppHandle, label: String) -> Result<(), AppError> {
    desktop::recycle_notepad_window(&app, &label)
}

#[tauri::command]
async fn open_tile_window(
    app: AppHandle,
    note_id: String,
    bounds: Option<desktop::WindowBounds>,
) -> Result<String, AppError> {
    desktop::open_tile_window(app, note_id, bounds).await
}

#[tauri::command]
async fn toggle_tile_window(
    app: AppHandle,
    note_id: String,
    bounds: Option<desktop::WindowBounds>,
) -> Result<bool, AppError> {
    desktop::toggle_tile_window(app, note_id, bounds).await
}

#[tauri::command]
async fn open_note_in_editor(app: AppHandle, note_id: String) -> Result<(), AppError> {
    desktop::show_main_window(&app)?;
    let _ = app.emit("open-note", &note_id);
    Ok(())
}

#[tauri::command]
fn request_main_window_close(app: AppHandle) -> Result<(), AppError> {
    desktop::request_main_window_close(&app)
}

#[tauri::command]
fn request_app_exit(app: AppHandle) -> Result<(), AppError> {
    desktop::request_app_exit(&app)
}

#[tauri::command]
fn report_app_quit_preparation(
    app: AppHandle,
    request_id: String,
    window_label: String,
    status: String,
    message: Option<String>,
) -> Result<(), AppError> {
    desktop::report_app_quit_preparation(&app, &request_id, &window_label, &status, message)
}

#[tauri::command]
fn take_startup_files() -> Vec<String> {
    desktop::take_startup_files()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if let Some(file_path) = desktop::extract_file_arg(&args) {
                desktop::handle_open_file_request(app, file_path);
            }
        }))
        .setup(|app| {
            if let Ok(store) = default_store() {
                let base = store.base_dir();
                let scope = app.asset_protocol_scope();
                let _ = scope.allow_directory(base.join("images"), true);
                let _ = scope.allow_directory(base.join("backgrounds"), true);
            }
            let updater_state = updater::UpdaterState::new(app.package_info().version.to_string());
            if let Err(error) = updater_state.initialize() {
                eprintln!("failed to initialize updater infrastructure: {error}");
            }
            app.manage(updater_state);
            desktop::setup_desktop(app)?;
            Ok(())
        })
        .on_window_event(desktop::handle_window_event)
        .invoke_handler(tauri::generate_handler![
            app_name,
            notes_list,
            notes_get,
            notes_create,
            notes_update,
            notes_delete,
            notes_import_markdown,
            notes_export_markdown,
            notes_sync_source_file,
            notes_reload_source_file,
            notes_move_category,
            categories_list,
            categories_create,
            categories_rename,
            categories_delete,
            images_save,
            images_get_base_dir,
            images_clean_unused,
            config_get,
            copy_background_image,
            config_save,
            global_shortcut_check,
            start_shortcut_recording,
            stop_shortcut_recording,
            open_notepad_window,
            recycle_notepad_window,
            open_tile_window,
            toggle_tile_window,
            open_note_in_editor,
            request_main_window_close,
            request_app_exit,
            report_app_quit_preparation,
            updater::commands::update_status,
            updater::commands::update_settings_get,
            updater::commands::update_settings_save,
            updater::commands::update_check,
            updater::commands::update_download,
            updater::commands::update_install,
            updater::commands::update_install_prepare_report,
            updater::commands::update_cancel,
            take_startup_files
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            #[cfg(any(target_os = "macos", target_os = "ios", target_os = "android"))]
            if let tauri::RunEvent::Opened { urls } = event {
                for url in urls {
                    if let Some(file_path) = desktop::extract_file_url(&url) {
                        desktop::handle_open_file_request(app, file_path);
                    }
                }
            }

            #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "android")))]
            let _ = (app, event);
        });
}
