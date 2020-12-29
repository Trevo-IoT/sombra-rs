use std::io::Error;
use std::str::Utf8Error;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;

#[derive(Debug, PartialEq)]
pub struct SombraError {
    pub description: String
}

impl From<std::io::Error> for SombraError {
    fn from(e: Error) -> Self {
        SombraError {
            description: e.to_string()
        }
    }
}

impl From<Utf8Error> for SombraError {
    fn from(e: Utf8Error) -> Self {
        SombraError {
            description: e.to_string()
        }
    }
}

pub trait Sombra {
    fn build(name: &str, path: &str, args: Vec<String>) -> Self;
    fn create(&self) -> Result<(), SombraError>;
    fn delete(&self) -> Result<(), SombraError>;
}

#[cfg(target_os = "windows")]
pub fn build(name: &str, path: &str, args: Vec<String>) -> windows::sombra_imp::SombraWindows {
    windows::sombra_imp::SombraWindows::build(name, path, args)
}

#[cfg(target_os = "linux")]
pub fn build(name: &str, path: &str, args: Vec<String>) -> linux::sombra_imp::SombraLinux {
    linux::sombra_imp::SombraLinux::build(name, path, args)
}