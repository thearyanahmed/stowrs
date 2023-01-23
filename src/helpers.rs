use std::any::Any;
use crate::adapters::local_file_system_adapter::LocalFileSystemAdapter;
use crate::adapters::s3_file_system_adapter::S3FileSystemAdapter;
use crate::storage::StorageAdapter;

pub fn get_adapter(name: &str, config: &dyn Any) -> Box<dyn StorageAdapter> {
    match name {
        "local" => {
            Box::new(LocalFileSystemAdapter::new(config))
        },
        "s3" => {
            Box::new(S3FileSystemAdapter::new(config))
        },
        _ => {
            panic!("this is a terrible mistake!");
        }
    }
}