use std::any::Any;
use crate::storage::{StorageAdapter, StorageAdapterConfig};

pub struct S3FileSystemAdapterConfig {
    pub bucket: String,
}

impl StorageAdapterConfig for S3FileSystemAdapterConfig {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> String {
        "s3".to_string()
    }
}

pub struct S3FileSystemAdapter {
    bucket: String
}

impl S3FileSystemAdapter {
    pub fn new(config: &dyn Any) -> S3FileSystemAdapter {

        let cfg : &S3FileSystemAdapterConfig = config.downcast_ref::<S3FileSystemAdapterConfig>().expect("failed to downcast");

        let base_dir = &cfg.bucket;

        S3FileSystemAdapter {
            bucket: base_dir.to_string(),
        }
    }
}

impl StorageAdapter for S3FileSystemAdapter {
    fn dir_exists(&self, path: String) -> bool {
        println!("from S3FileSystemAdapter impl block");
        false
    }
}