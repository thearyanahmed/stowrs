pub enum DirectoryError {
    InvalidPermissions,
}

impl ToString for DirectoryError {
    fn to_string(&self) -> String {
        match *self {
            DirectoryError::InvalidPermissions => "invalid permissions",
        }.to_string()
    }
}