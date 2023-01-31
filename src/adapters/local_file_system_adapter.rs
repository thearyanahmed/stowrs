use crate::storage::{StorageAdapter, StorageAdapterConfig};
use std::any::Any;
use std::env;
use std::path::{Path, PathBuf};

pub struct LocalFileSystemAdapterConfig {
    pub base_directory: String,
}

impl StorageAdapterConfig for LocalFileSystemAdapterConfig {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct LocalFileSystemAdapter {
    base_dir: String,
}

impl LocalFileSystemAdapter {
    pub fn new(config: &dyn Any) -> LocalFileSystemAdapter {
        let cfg: &LocalFileSystemAdapterConfig = config
            .downcast_ref::<LocalFileSystemAdapterConfig>()
            .expect("failed to downcast");

        let base_dir = &cfg.base_directory;

        LocalFileSystemAdapter {
            base_dir: base_dir.to_string(),
        }
    }

    // @ not sure if we need it
    fn get_base_dir(&self) -> String {
        match env::current_dir() {
            Ok(path_buf) => match path_buf.to_str() {
                None => self.get_fallback_base_dir(),
                Some(path_str) => path_str.to_string(),
            },
            Err(_) => self.get_fallback_base_dir(),
        }
    }

    fn get_full_path_buf(&self, path: String) -> PathBuf {
        let path_buf = self.to_base_path();

        path_buf.join(&path)
    }

    fn to_base_path(&self) -> &Path {
        Path::new(&self.base_dir)
    }

    fn get_fallback_base_dir(&self) -> String {
        "/".to_string()
    }
}

impl StorageAdapter for LocalFileSystemAdapter {
    fn disk_name(&self) -> String {
        "local".to_string()
    }

    fn dir_exists(&self, path: String) -> bool {
        println!("from LocalFileSystemAdapter impl block");
        false
    }
}
