use stowrs::{DemoStorageAdapterConfig, Hello, S3StorageAdapterConfig};
use stowrs::storage::{Storage, StorageAdapterConfigToBeRemoved, StorageConfig};

fn main() {
    println!("running example main");

    // let mut default_conf = StorageConfig::default();

    // let conf = default_conf.base_dir("/Users/thearyanahmed/rusted/stowrs".to_string());

    let conf = StorageAdapterConfigToBeRemoved{};

    let mut storage = Storage::new(conf);

    println!("storage exists: {}",storage.dir_exists("examples".to_string()));

    let _= storage.make_base_dir("/Users/thearyanahmed/rusted".to_string());

    let _demo_storage_config = DemoStorageAdapterConfig{};
    let s3_storage_config = S3StorageAdapterConfig{};

    let hello = Hello::new(s3_storage_config);

    hello.demo_dir_exists();
}