pub mod result;
pub mod error;

pub use result::Result;
pub use error::Error;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;

pub trait Sombra {
    fn build(name: &str, path: &str, args: Vec<String>) -> Self;
    fn create(&self) -> Result<()>;
    fn delete(&self) -> Result<()>;
}

#[cfg(target_os = "windows")]
pub fn build(name: &str, path: &str, args: Vec<String>) -> windows::sombra_imp::SombraWindows {
    windows::sombra_imp::SombraWindows::build(name, path, args)
}

#[cfg(target_os = "linux")]
pub fn build(name: &str, path: &str, args: Vec<String>) -> linux::sombra_imp::SombraLinux {
    linux::sombra_imp::SombraLinux::build(name, path, args)
}