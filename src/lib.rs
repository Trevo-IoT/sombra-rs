mod result;
mod error;

pub use result::Result;
pub use error::{Error, ErrorKind};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;

pub trait Sombra {
    fn build(name: &str, path: &str, args: Vec<String>) -> Result<Self>
        where Self: std::marker::Sized;
    fn create(&self) -> Result<()>;
    fn delete(&self) -> Result<()>;
}

#[cfg(target_os = "windows")]
pub fn build(name: &str, path: &str, args: Vec<String>) -> Result<windows::sombra_imp::SombraWindows> {
    windows::sombra_imp::SombraWindows::build(name, path, args)
}

#[cfg(target_os = "linux")]
pub fn build(name: &str, path: &str, args: Vec<String>) -> Result<linux::sombra_imp::SombraLinux> {
    linux::sombra_imp::SombraLinux::build(name, path, args)
}