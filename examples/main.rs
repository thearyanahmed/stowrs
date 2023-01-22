use stowrs::storage::Storage;

fn main() {
    println!("running example main");

    let mut storage = Storage::default();

    println!("storage exists: {}",storage.dir_exists("/Users/thearyanahmed/rusted/stowrs".to_string()));

    let _= storage.root("/Users/thearyanahmed/rusted".to_string());

    storage.get_root()
}