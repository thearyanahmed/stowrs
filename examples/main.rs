use std::any::Any;
use stowrs::storage::{Storage, StorageAdapter};
use stowrs::adapters::local_file_system_adapter::{LocalFileSystemAdapter, LocalFileSystemAdapterConfig};
use stowrs::adapters::s3_file_system_adapter::{S3FileSystemAdapter, S3FileSystemAdapterConfig};
use stowrs::helpers;

fn main() {
    println!("running example main");

    // let conf = StorageAdapterConfigToBeRemoved{};
    //
    // let mut storage = Storage::new(conf);
    //
    // println!("storage exists: {}",storage.dir_exists("examples".to_string()));
    //
    // let _= storage.make_base_dir("/Users/thearyanahmed/rusted".to_string());
    //
    // let _demo_storage_config = DemoStorageAdapterConfig{};
    // let s3_storage_config = S3StorageAdapterConfig{};
    //
    // let hello = Hello::new(s3_storage_config);
    //
    // hello.demo_dir_exists();

    // 0.1

    let local_adapter_config = LocalFileSystemAdapterConfig{ base_directory: "/Users/thearyanahmed/rusted".to_string() };
    let s3_storage_config = S3FileSystemAdapterConfig{ bucket: "hello-world".to_string() };

    let storage = Storage::new(helpers::get_adapter("local",&local_adapter_config));

    let v : Vec<Box<dyn StorageAdapter>> = vec![
        Box::new(LocalFileSystemAdapter::new(&local_adapter_config)),
        Box::new(S3FileSystemAdapter::new(&s3_storage_config)),
    ];

    Storage::init("local".to_string(), v);

    storage.dir_exists("a".to_string());
}
