use crate::errors::DirectoryError;
use crate::file::File;

pub struct Storage {
    root: String,
    dir_separator: String,
    disk: String,
}


impl Default for Storage {
    fn default() -> Self {
        Storage {
            root: "()".to_string(),
            dir_separator: "".to_string(),
            disk: "disk".to_string(),
        }
    }
}

impl Storage {
    pub fn new() -> Storage {
        Storage::default() // @note for now only
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

    pub fn dir_exists(&self, name: String) -> Result<bool, DirectoryError> {
        Err(DirectoryError::InvalidPermissions)
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
