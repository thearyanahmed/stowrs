use stowrs::storage::{Storage};
use stowrs::adapters::local_file_system_adapter::LocalFileSystemAdapterConfig;
use stowrs::adapters::s3_file_system_adapter::S3FileSystemAdapterConfig;

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
    let _s3_storage_config = S3FileSystemAdapterConfig{ bucket: "hello-world".to_string() };

    let storage = Storage::new(local_adapter_config);

    storage.dir_exists("a".to_string());
}