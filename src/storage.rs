use std::borrow::{Borrow, BorrowMut};
use std::env;
use std::fmt::format;
use crate::errors::DirectoryError;
use crate::file::File;
use std::path::{Path, PathBuf};


pub struct Storage {
    base_directory: String,
    disk: String,
}

pub trait StorageAdapter {

}

pub struct StorageConfig {
    pub base_directory: Option<String>,
    // pub disk_driver: dyn StorageAdapter,
}

impl Default for StorageConfig {
    fn default() -> Self {
        StorageConfig {
            base_directory: None,
        }
    }
}

impl StorageConfig {
    pub fn base_dir(&mut self, path: String) -> &mut StorageConfig {
        self.base_directory =  Some(path);

        &mut *self
    }
}


impl Default for Storage {
    fn default() -> Self {
        Storage {
            base_directory: Storage::get_default_base_directory(),
            disk: "disk".to_string(),
        }
    }
}

impl Storage {
    pub fn new(config: &StorageConfig) -> Storage {

        let dir = match &config.base_directory {
            None => Storage::get_default_base_directory(),
            Some(dir) => dir.to_string(),
        };

        Storage {
            base_directory: dir,
            disk: "/".to_string(),
        }
    }

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
        Path::new(&self.base_directory)
    }

    pub fn dir_exists(&self, path: String) -> bool {
        self.get_full_path_buf(path).is_dir()
    }

    pub fn make_base_dir(&mut self, path: String) -> &Storage {
        self.base_directory = path;

        &*self
    }

    // @todo delete
    pub fn get_root(&self) {
        println!("root: {}",self.base_directory)
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
        "list of all direcotires, from a root path"
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
