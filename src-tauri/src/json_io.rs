use crate::services::notes::AppError;
use serde::Serialize;
use std::{
    fs,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub fn write_json_atomic<T: Serialize>(path: &Path, value: &T) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let temp_path = temporary_json_path(path);
    let mut temp_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temp_path)?;
    let result = (|| {
        serde_json::to_writer_pretty(&mut temp_file, value)?;
        temp_file.write_all(b"\n")?;
        temp_file.sync_all()?;
        drop(temp_file);
        fs::rename(&temp_path, path)?;
        sync_parent_dir(path)?;
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }

    result
}

pub fn write_text_atomic(path: &Path, value: &str) -> Result<(), AppError> {
    write_bytes_atomic(path, value.as_bytes())
}

pub fn write_bytes_atomic(path: &Path, value: &[u8]) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let temp_path = temporary_path(path, "tmp");
    let mut temp_file = OpenOptions::new()
        .create(true)
        .write(true)
        .create_new(true)
        .open(&temp_path)?;
    let result = (|| {
        temp_file.write_all(value)?;
        temp_file.sync_all()?;
        drop(temp_file);
        fs::rename(&temp_path, path)?;
        sync_parent_dir(path)?;
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }

    result
}

fn temporary_json_path(path: &Path) -> PathBuf {
    temporary_path(path, "tmp")
}

fn temporary_path(path: &Path, suffix: &str) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("mininote");
    path.with_file_name(format!("{file_name}.{}.{suffix}", Uuid::new_v4()))
}

#[cfg(not(target_os = "windows"))]
fn sync_parent_dir(path: &Path) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        fs::File::open(parent)?.sync_all()?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn sync_parent_dir(_path: &Path) -> Result<(), AppError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, sync::Arc, thread};

    #[test]
    fn concurrent_text_writes_do_not_share_temp_file() {
        let root = std::env::temp_dir().join(format!("mininote-json-io-{}", Uuid::new_v4()));
        fs::create_dir_all(&root).expect("create temp root");
        let path = root.join("note.md");
        let values: Vec<String> = (0..32).map(|index| format!("content-{index}")).collect();
        let shared_values = Arc::new(values.clone());

        let handles: Vec<_> = (0..values.len())
            .map(|index| {
                let path = path.clone();
                let shared_values = Arc::clone(&shared_values);
                thread::spawn(move || write_text_atomic(&path, &shared_values[index]))
            })
            .collect();

        for handle in handles {
            handle.join().expect("join writer").expect("write succeeds");
        }

        let final_content = fs::read_to_string(&path).expect("read final content");
        assert!(values.contains(&final_content));
        let leftovers: Vec<_> = fs::read_dir(&root)
            .expect("read temp root")
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().contains(".tmp"))
            .collect();
        assert!(leftovers.is_empty());

        fs::remove_dir_all(root).expect("remove temp root");
    }
}
