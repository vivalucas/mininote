use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Listener};

use super::notes::{default_store, file_modified_time_ms, same_modified_time};

const DEBOUNCE_DURATION: Duration = Duration::from_millis(900);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceFileChangedPayload {
    pub note_id: String,
    pub title: String,
}

type SharedDebouncer = Arc<Mutex<notify_debouncer_mini::Debouncer<notify::RecommendedWatcher>>>;

pub struct SourceFileWatcher {
    debouncer: SharedDebouncer,
    watched_dirs: Arc<Mutex<HashSet<PathBuf>>>,
}

impl SourceFileWatcher {
    pub fn start(app: AppHandle) -> Self {
        let (tx, rx) = mpsc::channel();
        let debouncer = new_debouncer(DEBOUNCE_DURATION, tx).expect("failed to create debouncer");
        let debouncer = Arc::new(Mutex::new(debouncer));
        let watched_dirs: Arc<Mutex<HashSet<PathBuf>>> = Arc::new(Mutex::new(HashSet::new()));

        // Spawn a thread to process debounced file events
        let app_clone = app.clone();
        thread::spawn(move || {
            while let Ok(result) = rx.recv() {
                match result {
                    Ok(events) => {
                        for event in events {
                            if matches!(event.kind, DebouncedEventKind::Any) {
                                Self::handle_source_change(&app_clone, &event.path);
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!("source watcher error: {error}");
                    }
                }
            }
        });

        let watcher = Self {
            debouncer,
            watched_dirs,
        };

        // Initial watch setup
        if let Err(e) = watcher.refresh_watch_list() {
            eprintln!("failed to initialize source file watch list: {e}");
        }

        // Listen for notes-changed events to keep watch list in sync
        let debouncer_clone = Arc::clone(&watcher.debouncer);
        let dirs_clone = Arc::clone(&watcher.watched_dirs);
        app.listen("notes-changed", move |_event| {
            // Short delay to let metadata.json finish writing
            thread::sleep(Duration::from_millis(50));
            if let (Ok(mut debouncer), Ok(mut watched)) =
                (debouncer_clone.lock(), dirs_clone.lock())
            {
                let _ = refresh_watched_dirs(&mut debouncer, &mut watched);
            }
        });

        watcher
    }

    fn handle_source_change(app: &AppHandle, changed_path: &Path) {
        let store = match default_store() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("source watcher: failed to create store: {e}");
                return;
            }
        };

        let metadata_file = match store.load_metadata_for_watcher() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("source watcher: failed to load metadata: {e}");
                return;
            }
        };

        for note_meta in metadata_file.notes.iter() {
            let Some(source_path) = note_meta.source_path.as_ref() else {
                continue;
            };
            let source_path = PathBuf::from(source_path);
            if !source_path_may_match_event(&source_path, changed_path) {
                continue;
            }

            // Check if the file was actually modified (mtime changed).
            let current_mtime = match file_modified_time_ms(&source_path) {
                Ok(m) => m,
                Err(_) => continue, // File may have been deleted
            };

            if let Some(recorded_mtime) = note_meta.source_modified_time {
                if same_modified_time(current_mtime, recorded_mtime) {
                    continue;
                }
            }

            let payload = SourceFileChangedPayload {
                note_id: note_meta.id.clone(),
                title: note_meta.title.clone(),
            };
            let _ = app.emit("source-file-changed", payload);
        }
    }

    fn refresh_watch_list(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut debouncer = self.debouncer.lock().map_err(|e| e.to_string())?;
        let mut watched = self.watched_dirs.lock().map_err(|e| e.to_string())?;
        refresh_watched_dirs(&mut debouncer, &mut watched)
    }
}

fn source_path_may_match_event(source_path: &Path, changed_path: &Path) -> bool {
    let canonical_source = std::fs::canonicalize(source_path).ok();
    let canonical_changed = std::fs::canonicalize(changed_path).ok();
    match (canonical_source, canonical_changed) {
        (Some(source), Some(changed)) if source == changed => return true,
        _ => {}
    }

    if source_path == changed_path {
        return true;
    }

    match (source_path.parent(), changed_path.parent()) {
        (Some(source_parent), Some(changed_parent)) => source_parent == changed_parent,
        _ => false,
    }
}

fn refresh_watched_dirs(
    debouncer: &mut notify_debouncer_mini::Debouncer<notify::RecommendedWatcher>,
    watched: &mut HashSet<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let store = default_store()?;
    let metadata_file = store.load_metadata_for_watcher()?;

    let current_dirs: HashSet<PathBuf> = metadata_file
        .notes
        .iter()
        .filter_map(|note| {
            note.source_path.as_ref().and_then(|sp| {
                let path = PathBuf::from(sp);
                if path.is_file() {
                    path.parent().map(Path::to_path_buf)
                } else if path.parent().is_some_and(Path::is_dir) {
                    path.parent().map(Path::to_path_buf)
                } else {
                    None
                }
            })
        })
        .collect();

    // Remove watches for directories no longer in the source_path list.
    for dir in watched.iter() {
        if !current_dirs.contains(dir) {
            let _ = debouncer.watcher().unwatch(dir);
        }
    }

    // Watch parent directories rather than files so atomic rename-based writes
    // keep producing events after the source file is replaced.
    for dir in &current_dirs {
        if !watched.contains(dir) {
            if let Err(e) = debouncer.watcher().watch(dir, RecursiveMode::NonRecursive) {
                eprintln!("source watcher: failed to watch {}: {e}", dir.display());
            }
        }
    }

    *watched = current_dirs;
    Ok(())
}
