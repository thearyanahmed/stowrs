use stowrs::storage::Storage;

fn main() {
    println!("running example main");

    let storage = Storage::default();

    match storage.dir_exists("hello".to_string()) {
        Ok(_) => {
            println!("exists")
        }
        Err(err) => println!("{}", err.to_string())
    }
}