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
    watched_paths: Arc<Mutex<HashSet<PathBuf>>>,
}

impl SourceFileWatcher {
    pub fn start(app: AppHandle) -> Self {
        let (tx, rx) = mpsc::channel();
        let debouncer = new_debouncer(DEBOUNCE_DURATION, tx).expect("failed to create debouncer");
        let debouncer = Arc::new(Mutex::new(debouncer));
        let watched_paths: Arc<Mutex<HashSet<PathBuf>>> = Arc::new(Mutex::new(HashSet::new()));

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
            watched_paths,
        };

        // Initial watch setup
        if let Err(e) = watcher.refresh_watch_list() {
            eprintln!("failed to initialize source file watch list: {e}");
        }

        // Listen for notes-changed events to keep watch list in sync
        let debouncer_clone = Arc::clone(&watcher.debouncer);
        let paths_clone = Arc::clone(&watcher.watched_paths);
        app.listen("notes-changed", move |_event| {
            // Short delay to let metadata.json finish writing
            thread::sleep(Duration::from_millis(50));
            if let (Ok(mut debouncer), Ok(mut watched)) =
                (debouncer_clone.lock(), paths_clone.lock())
            {
                let _ = refresh_watched_paths(&mut debouncer, &mut watched);
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

        // Find the note whose source_path matches the changed file.
        // Canonicalize both paths to handle differences in separators,
        // symlinks, and relative components across platforms.
        let canonical_changed = std::fs::canonicalize(changed_path).ok();
        let note_meta = metadata_file.notes.iter().find(|note| {
            note.source_path
                .as_ref()
                .and_then(|sp| {
                    let note_path = PathBuf::from(sp);
                    let canonical_note = std::fs::canonicalize(&note_path).ok();
                    match (canonical_note, canonical_changed.as_ref()) {
                        (Some(a), Some(b)) => Some(a == *b),
                        _ => Some(note_path == changed_path),
                    }
                })
                .unwrap_or(false)
        });

        let note_meta = match note_meta {
            Some(n) => n,
            None => return,
        };

        let note_id = &note_meta.id;

        // Check if the file was actually modified (mtime changed)
        let current_mtime = match file_modified_time_ms(changed_path) {
            Ok(m) => m,
            Err(_) => return, // File may have been deleted
        };

        if let Some(recorded_mtime) = note_meta.source_modified_time {
            if same_modified_time(current_mtime, recorded_mtime) {
                return; // No real change
            }
        }

        // Reload the source file into MiniNote
        match store.reload_source_file(note_id) {
            Ok(reloaded) => {
                let payload = SourceFileChangedPayload {
                    note_id: reloaded.id,
                    title: reloaded.title,
                };
                let _ = app.emit("source-file-changed", payload);
                // Also notify the sidebar to refresh metadata (updatedAt, preview, etc.)
                let _ = app.emit("notes-changed", ());
            }
            Err(e) => {
                eprintln!("source watcher: failed to reload note {}: {e}", note_id);
            }
        }
    }

    fn refresh_watch_list(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut debouncer = self.debouncer.lock().map_err(|e| e.to_string())?;
        let mut watched = self.watched_paths.lock().map_err(|e| e.to_string())?;
        refresh_watched_paths(&mut debouncer, &mut watched)
    }
}

fn refresh_watched_paths(
    debouncer: &mut notify_debouncer_mini::Debouncer<notify::RecommendedWatcher>,
    watched: &mut HashSet<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let store = default_store()?;
    let metadata_file = store.load_metadata_for_watcher()?;

    let current_paths: HashSet<PathBuf> = metadata_file
        .notes
        .iter()
        .filter_map(|note| {
            note.source_path.as_ref().and_then(|sp| {
                let path = PathBuf::from(sp);
                if path.is_file() {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();

    // Remove watches for paths no longer in source_path list
    for path in watched.iter() {
        if !current_paths.contains(path) {
            let _ = debouncer.watcher().unwatch(path);
        }
    }

    // Add watches for new paths
    for path in &current_paths {
        if !watched.contains(path) {
            if let Err(e) = debouncer.watcher().watch(path, RecursiveMode::NonRecursive) {
                eprintln!("source watcher: failed to watch {}: {e}", path.display());
            }
        }
    }

    *watched = current_paths;
    Ok(())
}
