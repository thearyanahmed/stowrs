use std::any::Any;
use stowrs::adapters::local_file_system_adapter::{
    LocalFileSystemAdapter, LocalFileSystemAdapterConfig,
};
use stowrs::adapters::s3_file_system_adapter::{S3FileSystemAdapter, S3FileSystemAdapterConfig};
use stowrs::helpers;
use stowrs::storage::{Storage, StorageAdapter};

fn main() {
    println!("running example main");

    let local_adapter_config = LocalFileSystemAdapterConfig {
        base_directory: "/Users/thearyanahmed/rusted".to_string(),
    };
    let s3_storage_config = S3FileSystemAdapterConfig {
        bucket: "hello-world".to_string(),
    };

    let storage = Storage::new(helpers::get_adapter("local", &local_adapter_config));

    let v: Vec<Box<dyn StorageAdapter>> = vec![
        Box::new(LocalFileSystemAdapter::new(&local_adapter_config)),
        Box::new(S3FileSystemAdapter::new(&s3_storage_config)),
    ];

    Storage::init("local".to_string(), v);

    storage.dir_exists("a".to_string());
}
