use crate::Sombra;
use std::ffi::{OsString, OsStr};
use windows_service::{
    service::{ServiceAccess, ServiceState, ServiceErrorControl, ServiceInfo,
              ServiceStartType, ServiceType},
    service_manager::{ServiceManager, ServiceManagerAccess}
};
use std::time::Duration;
use std::path::PathBuf;

pub struct SombraWindows {
    process_path: PathBuf,
    process_name: String,
    process_args: Vec<String>,
}

impl Sombra for SombraWindows {
    fn build(name: &str, path: &str, args: Vec<String>) -> Self {
        let path = dunce::canonicalize(path)
            .expect(&format!("Cannot find {}", path));
        SombraWindows {
            process_path: path,
            process_name: name.to_string(),
            process_args: args,
        }
    }

    fn create(&self) -> crate::Result<()> {
        let manager_access = ServiceManagerAccess::CONNECT |
            ServiceManagerAccess::CREATE_SERVICE;
        let service_manager = ServiceManager::local_computer(None::<&str>,
                                                             manager_access)?;
        if std::env::var("SOMBRA_WINDOWS_SERVICE_PATH").is_err() {
            std::env::set_var("SOMBRA_WINDOWS_SERVICE_PATH",
                              "executables/sombra-windows-service.exe");
        }
        let sombra_win_service = match std::env::var("SOMBRA_WINDOWS_SERVICE_PATH") {
            Ok(o) => o,
            Err(_) => {
                return Err(crate::Error::IO(
                    "Environment variable SOMBRA_WINDOWS_SERVICE_PATH not found".to_string()))
            }
        };

        let service_binary_path = dunce::canonicalize(&sombra_win_service)
            .expect(&format!("Cannot find {}", &sombra_win_service));
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
        let mut args = vec![OsStr::new(&self.process_path)];
        for a in &self.process_args {
            args.push(a.as_ref());
        }
        service.start(&args)?;

        Ok(())
    }

    fn delete(&self) -> crate::Result<()> {
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
            std::thread::sleep(Duration::from_millis(100))
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

    fn echo_check(ip_port: &str, msg: &[u8]) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(ip_port)?;
        stream.write(msg)?;
        let mut buffer = [0u8; 512];
        stream.read(&mut buffer);
        let mut buffer = buffer.to_vec();
        buffer.retain(|&x| x != 0);
        if buffer != msg.to_vec() {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Not match"))
        } else {
            Ok(())
        }
    }

    #[test]
    fn spawn_simple() {
        let s = SombraWindows::build("tcp_echo",
                                     "executables/tcp_echo.exe", vec![]);
        assert_eq!(s.create(), Ok(()));
        let res = echo_check("127.0.0.1:30222", b"sombra30222");
        assert_eq!(s.delete(), Ok(()));
        if let Err(e) = res {
            panic!(format!("{:?}", e));
        }
    }

    #[test]
    fn spawn_twice_same_name() {
        let s = SombraWindows::build("tcp_echo",
                                     "executables/tcp_echo.exe", vec![]);
        assert_eq!(s.create(), Ok(()));
        match echo_check("127.0.0.1:30222", b"sombra30222") {
            Ok(_) => {
                let s2 = SombraWindows::build("tcp_echo",
                                              "executables/tcp_echo.exe", vec![]);
                assert_ne!(s2.create(), Ok(()));
                assert_eq!(s.delete(), Ok(()));
            },
            Err(e) => {
                assert_eq!(s.delete(), Ok(()));
                panic!(format!("{:?}", e));
            }
        }
    }

    #[test]
    fn spawn_twice_other_name() {
        let s = SombraWindows::build("tcp_echo30222",
                                     "executables/tcp_echo.exe",
                                     vec!["-p".to_string(), "30222".to_string()]);
        assert_eq!(s.create(), Ok(()));

        match echo_check("127.0.0.1:30222", b"sombra30222") {
            Ok(_) => {
                let s2 = SombraWindows::build("tcp_echo30223",
                                              "executables/tcp_echo.exe",
                                              vec!["-p".to_string(), "30223".to_string()]);
                assert_eq!(s2.create(), Ok(()));
                match echo_check("127.0.0.1:30223", b"sombra30223") {
                    Ok(_) => {
                        assert_eq!(s.delete(), Ok(()));
                        assert_eq!(s2.delete(), Ok(()));
                    },
                    Err(e) => {
                        assert_eq!(s.delete(), Ok(()));
                        assert_eq!(s2.delete(), Ok(()));
                        panic!(format!("{:?}", e));
                    },
                }
            },
            Err(e) => {
                assert_eq!(s.delete(), Ok(()));
                panic!(format!("{:?}", e));
            }
        }
    }

    #[test]
    fn spawn_with_args() {
        let s = SombraWindows::build("tcp_echo",
                                     "executables/tcp_echo.exe",
                                     vec!["-p".to_string(), "30223".to_string()]);
        assert_eq!(s.create(), Ok(()));
        let res = echo_check("127.0.0.1:30223", b"sombra30223");
        assert_eq!(s.delete(), Ok(()));
        if let Err(e) = res {
            panic!(format!("{:?}", e));
        }
    }

    #[test]
    fn spawn_once_delete_twice() {
        let s = SombraWindows::build("tcp_echo",
                                     "executables/tcp_echo.exe", vec![]);
        assert_eq!(s.create(), Ok(()));
        match echo_check("127.0.0.1:30222", b"sombra30222") {
            Ok(_) => {
                assert_eq!(s.delete(), Ok(()));
                assert_ne!(s.delete(), Ok(()));
            },
            Err(e) => {
                assert_eq!(s.delete(), Ok(()));
                panic!(format!("{:?}", e));
            }
        }
    }

    #[test]
    fn spawn_bug_and_correct() {
        let s = SombraWindows::build("tcp_echo",
                                     "executables/tcp_echo.exe", vec![]);
        assert_eq!(s.create(), Ok(()));
        match echo_check("127.0.0.1:30222", b"bug") {
            Ok(_) => {
                assert_eq!(s.delete(), Ok(()));
                assert_eq!(s.create(), Ok(()));
                match echo_check("127.0.0.1:30222", b"sombra30222") {
                    Ok(_) => {
                        assert_eq!(s.delete(), Ok(()));
                    },
                    Err(e) => {
                        assert_eq!(s.delete(), Ok(()));
                        panic!(format!("{:?}", e));
                    }
                }
            },
            Err(e) => {
                assert_eq!(s.delete(), Ok(()));
                panic!(format!("{:?}", e));
            }
        }
    }
}
