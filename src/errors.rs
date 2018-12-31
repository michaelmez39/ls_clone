pub enum Error {
    path_error,
    generic_error
}

impl Error {
    pub fn get_err(&self) -> &str{
        match self {
            path_error => "Path error",
            generic_error => "Don't know what wen wrong"
        }
    }
}