use crate::storage::{StorageAdapter};

pub struct LocalFileSystemAdapter {

}

impl StorageAdapter for LocalFileSystemAdapter {
    fn dir_exists(&self, path: String) -> bool {
        todo!()
    }
}