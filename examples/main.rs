use stowrs::storage::{Storage, StorageConfig};

fn main() {
    println!("running example main");

    let mut default_conf = StorageConfig::default();

    let conf = default_conf.base_dir("/Users/thearyanahmed/rusted/stowrs".to_string());

    let mut storage = Storage::new(&conf);

    println!("storage exists: {}",storage.dir_exists("examples".to_string()));

    let _= storage.make_base_dir("/Users/thearyanahmed/rusted".to_string());

    storage.get_root()
}