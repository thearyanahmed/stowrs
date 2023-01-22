use std::env;
use crate::errors::DirectoryError;
use crate::file::File;
use std::path::{Path, PathBuf};

pub struct Storage {
    config: Box<dyn StorageAdapterConfig>
}

pub trait StorageAdapter {
    fn dir_exists(&self, path: String) -> bool;
}

pub trait StorageAdapterConfig {}

pub struct StorageAdapterConfigToBeRemoved {}

impl StorageAdapterConfig for StorageAdapterConfigToBeRemoved {}

impl Default for Storage {
    fn default() -> Self {

        let adapter_config = StorageAdapterConfigToBeRemoved{};
        let config = Box::new(adapter_config);

        Storage {
            // base_directory: Storage::get_default_base_directory(),
            // disk: "disk".to_string(),
            config,
        }
    }
}

impl Storage {
    pub fn new<T : StorageAdapterConfig + 'static>(conf: T) -> Storage {
        let config = Box::new(conf);

        Storage {
            // base_directory: "base_dir_string".to_string(),
            // disk: "/".to_string(),
            config,
        }
    }

    // pub fn old_new(config: &StorageConfig) -> Storage {
    //
    //     let dir = match &config.base_directory {
    //         None => Storage::get_default_base_directory(),
    //         Some(dir) => dir.to_string(),
    //     };
    //
    //     let adapter_config = StorageAdapterConfigToBeRemoved{};
    //     let config = Box::new(adapter_config);
    //
    //     Storage {
    //         base_directory: dir,
    //         disk: "/".to_string(),
    //         config,
    //     }
    // }

    fn get_default_base_directory() -> String {
        // @note it should be different for each driver

        match env::current_dir() {
            Ok(path_buf) => {
                match path_buf.to_str() {
                    None => "/".to_string(),
                    Some(path_str) => path_str.to_string()
                }
            },
            Err(_) => "/".to_string(),
        }
    }

    fn get_full_path_buf(&self, path: String) -> PathBuf {
        let path_buf = self.to_base_path();

        path_buf.join(&path)
    }

    fn to_base_path(&self) -> &Path {
        let x = "/Users/thearyanahmed/rusted";
        Path::new(x)
    }

    pub fn dir_exists(&self, path: String) -> bool {
        // should be something like
        // self.disk_driver.dir_exist()
        self.get_full_path_buf(path).is_dir()
    }

    pub fn make_base_dir(&mut self, path: String) -> &Storage {
        // self.base_directory = path;

        &*self
    }

    pub fn disk(&mut self) -> &mut Storage {
        // should return self
        self
    }

    pub fn put(name: String, content: String) {
        // stores a file
    }

    pub fn get(&self, name: String) {
        println!("looking for {}",name)
    }


    pub fn exists(&self, name: String) -> Result<bool, DirectoryError> {
        Ok(false)
    }

    // Options -> optional something, like headers, download as
    pub fn download(&self, name: String, options: String) -> &str {
        "returns stream"
    }

    pub fn url(&self, name: String) -> &str {
        "url"
    }

    // time should be time.time
    // Options -> optional something, like headers, download as
    pub fn temp_url(&self, name: String, time: i32, options: String) -> &str {
        "temporary url"
    }

    pub fn move_to(&mut self, file: &File, location: String) -> &str {
        "moving to a new location"
    }

    pub fn copy(&mut self, file: &File, location: String) -> &str {
        "moving to a new location"
    }

    pub fn delete(&self, file: &File) -> &str {
        "delete file, not sure about if file should be &Reference"
    }

    pub fn delete_from_path(&self, name: String) -> &str {
        "file does not exists"
    }

    pub fn make_directory(&self, name: String) -> &str {
        "check if dir exists, check if that directory is writable, try to create dir"
    }

    pub fn all_directories(&self) -> &str {
        "list of all directories, from a root path"
    }

    pub fn delete_directory(&self) -> bool {
        false
    }
}


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
