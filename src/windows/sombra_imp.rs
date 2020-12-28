use crate::{Sombra, SombraError};
use std::ffi::{OsString, OsStr};
use windows_service::{service::{ServiceAccess, ServiceState, ServiceErrorControl, ServiceInfo,
                                ServiceStartType, ServiceType}, service_manager::{ServiceManager, ServiceManagerAccess}, Error};
use std::time::Duration;
use std::path::PathBuf;

pub struct SombraWindows {
    process_path: PathBuf,
    process_name: String,
    auto_reload: bool,
}

impl SombraWindows {
    pub fn with_autoreload(mut self) -> Self {
        self.auto_reload = true;
        self
    }
}

impl std::convert::From<windows_service::Error> for SombraError {
    fn from(e: Error) -> Self {
        SombraError {
            description: format!("windows_service error: {:?}", e)
        }
    }
}

impl Sombra for SombraWindows {
    fn build(name: &str, path: &str) -> Self {
        let path = dunce::canonicalize(path)
            .expect(&format!("Cannot find {}", path));
        SombraWindows {
            process_path: path,
            process_name: name.to_string(),
            auto_reload: false,
        }
    }

    fn create(&self) -> Result<(), SombraError> {
        let manager_access = ServiceManagerAccess::CONNECT |
            ServiceManagerAccess::CREATE_SERVICE;
        let service_manager = ServiceManager::local_computer(None::<&str>,
                                                             manager_access)?;
        let sombra_win_service = "executables/sombra-windows-service.exe";
        let service_binary_path = dunce::canonicalize(sombra_win_service)
            .expect(&format!("Cannot find {}", sombra_win_service));
        let service_info = ServiceInfo {
            name: OsString::from(self.process_name.clone()),
            display_name: OsString::from(self.process_name.clone()),
            service_type: ServiceType::OWN_PROCESS,
            start_type: ServiceStartType::OnDemand,
            error_control: ServiceErrorControl::Normal,
            executable_path: PathBuf::from(service_binary_path),
            launch_arguments: vec![],
            dependencies: vec![],
            account_name: None, // run as System
            account_password: None,
        };
        let service = service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)?;
        service.set_description(format!("Sombra Service Wrapper on {}", self.process_name))?;

        let service_access = ServiceAccess::START;
        let service = service_manager.open_service(&self.process_name,
                                                   service_access)?;
        let args = [OsStr::new(&self.process_path)];
        service.start(&args)?;

        Ok(())
    }

    fn delete(&self) -> Result<(), SombraError> {
        let manager_access = ServiceManagerAccess::CONNECT;
        let service_manager = ServiceManager::local_computer(None::<&str>,
                                                             manager_access)?;
        let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP |
            ServiceAccess::DELETE;
        let service = service_manager.open_service(&self.process_name,
                                                   service_access)?;
        let service_status = service.query_status()?;
        if service_status.current_state != ServiceState::Stopped {
            service.stop()?;
            std::thread::sleep(Duration::from_secs(1))
        }

        service.delete()?;

        Ok(())
    }
}

#[cfg(test)]
#[cfg(target_os = "windows")]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::net::TcpStream;

    #[test]
    fn build() {
        let path = "executables/tcp_echo.exe";
        let s = SombraWindows::build("tcp_echo", path);
        assert_eq!(s.process_path, dunce::canonicalize(path).unwrap());
        assert_eq!(s.process_name, "tcp_echo");
    }

    #[test]
    fn spawn_simple() {
        let s = SombraWindows::build("tcp_echo", "executables/tcp_echo.exe");
        assert_eq!(s.create(), Ok(()));
        let stream = TcpStream::connect("127.0.0.1:30222");
        let msg_to_echo = b"sombra30222";

        match stream {
            Ok(mut stream) => {
                if let Ok(_) = stream.write(msg_to_echo) {
                    println!("Write ok");
                    let mut buffer = [0u8; 512];
                    match stream.read(&mut buffer) {
                        Ok(n) => {
                            let mut buffer = buffer.to_vec();
                            buffer.retain(|&x| x != 0);
                            if buffer != msg_to_echo.to_vec() {
                                let e = s.delete();
                                println!("delete result: {:?}", e);
                            }
                            assert_eq!(buffer, msg_to_echo.to_vec());
                        },
                        Err(e) => {
                            let e2 = s.delete();
                            panic!(format!("tcp stream read error: {:?}, {:?}", e, e2));
                        }
                    }
                } else {
                    let e = s.delete();
                    panic!(format!("tcp stream write error: {:?}", e));
                }
            },
            Err(e) => {
                let e2 = s.delete();
                panic!(format!("tcp stream error: {:?}, {:?}", e, e2));
            }
        }

        assert_eq!(Ok(()), s.delete());
    }

    #[test]
    fn spawn_twice() {
        unimplemented!()
    }

    #[test]
    fn spawn_once_delete_twice() {
        unimplemented!()
    }

    #[test]
    fn spawn_buggy_without_auto_reload() {
        unimplemented!()
    }

    #[test]
    fn spawn_buggy_with_auto_reload() {
        unimplemented!()
    }
}
