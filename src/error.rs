#[derive(Debug, PartialEq)]
pub enum Error {
    IO(String),
    Utf8(String),
    WindowsService(String),
    EnvVar(String),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::IO(s) => format!("[IO] {}", s.to_string()),
            Error::Utf8(s) => format!("[Utf8] {}", s.to_string()),
            Error::WindowsService(s) => format!("[WindowsService] {}", s.to_string()),
            Error::EnvVar(s) => format!("[EnvVar] {}", s.to_string()),
        }
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::EnvVar(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Error::Utf8(e.to_string())
    }
}

#[cfg(target_os = "windows")]
impl std::convert::From<windows_service::Error> for Error {
    fn from(e: windows_service::Error) -> Self {
        match e {
            windows_service::Error::Winapi(err) => Error::WindowsService(err.to_string()),
            _ => Error::WindowsService(e.to_string())
        }
    }
}
