use std::any::Any;
use crate::storage::{StorageAdapter, StorageAdapterConfig};

pub struct LocalFileSystemAdapterConfig {
    pub base_directory: String,
}

impl StorageAdapterConfig for LocalFileSystemAdapterConfig {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct LocalFileSystemAdapter {
    base_dir: String
}

impl LocalFileSystemAdapter {
    pub fn new(config: &dyn Any) -> LocalFileSystemAdapter {

        let cfg : &LocalFileSystemAdapterConfig = config.downcast_ref::<LocalFileSystemAdapterConfig>().expect("failed to downcast");

        let base_dir = &cfg.base_directory;

        LocalFileSystemAdapter {
            base_dir: base_dir.to_string(),
        }
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