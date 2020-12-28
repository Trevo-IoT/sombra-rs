use crate::{Sombra, SombraError};
use std::path::PathBuf;
use std::io::Write;
use crate::linux::systemctl::Systemctl;

pub struct SombraLinux {
    process_path: PathBuf,
    process_name: String,
    sysctl: Systemctl
}

impl SombraLinux {
    fn service_file_content(name: &str, path: &PathBuf) -> String {
        format!("[Unit]\n\
                Description={} service\n\
                After=network.target\n\
                StartLimitIntervalSec=0\n\
                \n\
                [Service]\n\
                Type=simple\n\
                User={}\n\
                ExecStart={}\n\
                \n\
                [Install]\n\
                WantedBy=multi-user.target",
                name,
                whoami::username(),
                path.to_str().unwrap())
    }

    fn is_root() -> Result<(), SombraError> {
        match std::env::var("USER") {
            Err(e) => Err(SombraError {
                description: e.to_string()
            }),
            Ok(name) => {
                if name != "root" {
                    Err(SombraError {
                        description: "Without root privileges.".to_string()
                    })
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl Sombra for SombraLinux {
    fn build(name: &str, path: &str) -> Self {
        let path = dunce::canonicalize(path).expect(&format!("Cannot find {}", path));
        SombraLinux {
            process_path: path,
            process_name: name.to_string(),
            sysctl: Systemctl::new(name)
        }
    }

    fn create(&self) -> Result<(), SombraError> {
        SombraLinux::is_root()?;

        let path = std::path::PathBuf::from(
            format!("/etc/systemd/system/{}.service", self.process_name));
        if path.exists() {
            return Err(SombraError {
                description: format!("Service {} already exist",
                                     self.process_name)
            });
        } else {
            let mut file = std::fs::File::create(&path)?;
            let buffer = SombraLinux::service_file_content(&self.process_name,
                                                               &self.process_path);
            file.write_all(&mut buffer.as_bytes())?;
        }

        self.sysctl.start()
    }

    fn delete(&self) -> Result<(), SombraError> {
        let _ = self.sysctl.stop();
        self.sysctl.disable()?;
        std::fs::remove_file(format!("/etc/systemd/system/{}.service", self.process_name))?;
        Systemctl::daemon_reload()?;
        Systemctl::reset_failed()
    }
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests {
    use super::*;
    use std::net::TcpStream;
    use std::io::Read;
    use std::time::Duration;

    fn echo_check(ip_port: &str, msg: &[u8]) -> std::io::Result<()> {
        std::thread::sleep(Duration::from_millis(1)); // Need to allow tcp_echo open TCP connection

        let mut stream = TcpStream::connect(ip_port)?;
        stream.write(msg)?;
        let mut buffer = [0u8; 512];
        stream.read(&mut buffer)?;
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
        let s = SombraLinux::build("tcp_echo", "executables/tcp_echo");
        assert_eq!(s.create(), Ok(()));
        let res = echo_check("127.0.0.1:30222", b"sombra30222");
        assert_eq!(s.delete(), Ok(()));
        if let Err(e) = res {
            panic!(format!("{:?}", e));
        }
    }

    #[test]
    fn spawn_twice_same_name() {
        let s = SombraLinux::build("tcp_echo", "executables/tcp_echo");
        assert_eq!(s.create(), Ok(()));
        match echo_check("127.0.0.1:30222", b"sombra30222") {
            Ok(_) => {
                let s2 = SombraLinux::build("tcp_echo", "executables/tcp_echo");
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
        unimplemented!();
    }

    #[test]
    fn spawn_once_delete_twice() {
        let s = SombraLinux::build("tcp_echo", "executables/tcp_echo");
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
        let s = SombraLinux::build("tcp_echo", "executables/tcp_echo");
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
