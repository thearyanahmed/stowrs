use crate::storage::StorageAdapter;

pub mod file;
pub mod storage;
pub mod errors;
mod adapters;


pub struct Hello  {
    driver_config: Box<dyn StorageAdapter>,
}

pub struct DemoStorageAdapterConfig {}
pub struct S3StorageAdapterConfig {}

impl StorageAdapter for DemoStorageAdapterConfig {
    fn dir_exists(&self, path: String) -> bool {
        println!("demo storage adapter");
        false
    }
}

impl StorageAdapter for S3StorageAdapterConfig {
    fn dir_exists(&self, path: String) -> bool {
        println!("s3 storage adapter");

        false
    }
}

impl Hello {
    pub fn new<T : StorageAdapter + 'static>(conf: T) -> Hello {
        let driver_conf = Box::new(conf);

        Hello {
            driver_config: driver_conf
        }
    }

    pub fn demo_dir_exists(&self) {
        self.driver_config.dir_exists("hello".to_string());
    }
}