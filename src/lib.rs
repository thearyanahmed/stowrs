pub struct Storage {
}

pub struct File {
    
}

impl Storage {
    pub fn disk(&mut self) -> &mut Storage {
        // should return self
        self
    }

    pub fn put(name: String, content: String) {
        // stores a file
    }

    pub fn get(&self, name: String) {

    }

    pub fn exists(&self, name: String) -> bool {
        false
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
        if self.exists(name) {
            return "try to delete";
        }

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

impl File {
    pub fn size(&self) -> i64 {
        100
    }

    pub fn mime(&self) -> i64 {
        100
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
