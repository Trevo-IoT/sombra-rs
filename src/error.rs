#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    desc: String,
    content: Option<String>,
}

impl Error {
    pub fn new(kind: ErrorKind, desc: String) -> Self {
        Error {
            kind,
            desc,
            content: None,
        }
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Other,
    Io,
    Utf8,
    WindowsService,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        if let Some(content) = &self.content {
            format!("<{:?}> {}: {}", self.kind, content, self.desc)
        } else {
            format!("<{:?}> {}", self.kind, self.desc)
        }
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::new(ErrorKind::Other, e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::new(ErrorKind::Other, e.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Error::new(ErrorKind::Other, e.to_string())
    }
}

#[cfg(target_os = "windows")]
impl std::convert::From<windows_service::Error> for Error {
    fn from(e: windows_service::Error) -> Self {
        match e {
            windows_service::Error::Winapi(err) => Error::new(ErrorKind::Other,
                                                              err.to_string()),
            _ => Error::new(ErrorKind::Other, e.to_string())
        }
    }
}
