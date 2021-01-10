use crate::Sombra;
use std::path::PathBuf;
use std::io::Write;
use crate::linux::systemctl::Systemctl;
use crate::error::ErrorKind::Other;

pub struct SombraLinux {
    process_path: PathBuf,
    process_name: String,
    process_args: Vec<String>,
    sysctl: Systemctl,
}

impl SombraLinux {
    fn service_file_content(name: &str, path: &PathBuf, args: &Vec<String>) -> crate::Result<String> {
        let path_str = match path.to_str() {
            Some(path_str) => path_str.to_string(),
            None => return Err(crate::Error::new(crate::ErrorKind::Io,
                                                 "Cannot decode path".to_string()))
        };

        let exec_start = if args.is_empty() {
            path_str
        } else {
            format!("{} {}", path_str, args.join(" "))
        };
        Ok(format!("[Unit]\n\
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
                exec_start))
    }

    fn is_root() -> crate::Result<()> {
        match std::env::var("USER") {
            Err(e) => Err(crate::Error::new(Other, e.to_string())),
            Ok(name) => {
                if name != "root" {
                    Err(crate::Error::new(Other,
                                          "Without root privileges.".to_string()))
                } else {
                    Ok(())
                }
            }
        }
    }
}

macro_rules! sombra_error {
    ($kind:ident, $content:expr) => {
        |e| crate::Error::new(crate::ErrorKind::$kind, e.to_string()).content($content)
    };
}

impl Sombra for SombraLinux {
    fn build(name: &str, path: &str, args: Vec<String>) -> crate::Result<Self> {
        let path = dunce::canonicalize(path)
            .map_err(sombra_error!(Io, path.to_string()))?;

        Ok(SombraLinux {
            process_path: path,
            process_name: name.to_string(),
            process_args: args,
            sysctl: Systemctl::new(name)
        })
    }

    fn create(&self) -> crate::Result<()> {
        SombraLinux::is_root()?;

        let path = std::path::PathBuf::from(
            format!("/etc/systemd/system/{}.service", self.process_name));
        if path.exists() {
            return Err(crate::Error::new(crate::ErrorKind::Io, format!("Service {} already exist",
                                     self.process_name)));
        } else {
            let mut file = std::fs::File::create(&path)?;
            let buffer = SombraLinux::service_file_content(&self.process_name,
                                                           &self.process_path,
                                                           &self.process_args)?;
            file.write_all(&mut buffer.as_bytes())?;
        }

        self.sysctl.start()
    }

    fn delete(&self) -> crate::Result<()> {
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
        std::thread::sleep(Duration::from_millis(10)); // Need to allow tcp_echo open TCP connection

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
        let s = match SombraLinux::build("tcp_echo", "executables/tcp_echo", vec![]) {
            Ok(s) => s,
            Err(e) => panic!(e.to_string()),
        };
        assert_eq!(s.create(), Ok(()));
        let res = echo_check("127.0.0.1:30222", b"sombra30222");
        assert_eq!(s.delete(), Ok(()));
        if let Err(e) = res {
            panic!(format!("{:?}", e));
        }
    }

    #[test]
    fn spawn_twice_same_name() {
        let s = match SombraLinux::build("tcp_echo", "executables/tcp_echo", vec![]) {
            Ok(s) => s,
            Err(e) => panic!(e.to_string()),
        };
        assert_eq!(s.create(), Ok(()));

        match echo_check("127.0.0.1:30222", b"sombra30222") {
            Ok(_) => {
                let s2 = match SombraLinux::build("tcp_echo", "executables/tcp_echo", vec![]) {
                    Ok(s2) => s2,
                    Err(e) => panic!(e.to_string()),
                };
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
        let s = match SombraLinux::build("tcp_echo30222",
                                     "executables/tcp_echo",
                                     vec!["-p".to_string(), "30222".to_string()]) {
            Ok(s) => s,
            Err(e) => panic!(e.to_string()),
        };
        assert_eq!(s.create(), Ok(()));

        match echo_check("127.0.0.1:30222", b"sombra30222") {
            Ok(_) => {
                let s2 = match SombraLinux::build("tcp_echo30223",
                                              "executables/tcp_echo",
                                              vec!["-p".to_string(), "30223".to_string()]) {
                    Ok(s) => s,
                    Err(e) => panic!(e.to_string()),
                };
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
        let s = match SombraLinux::build("tcp_echo",
                                     "executables/tcp_echo",
                                     vec!["-p".to_string(), "30223".to_string()]) {
            Ok(s) => s,
            Err(e) => panic!(e.to_string()),
        };
        assert_eq!(s.create(), Ok(()));
        let res = echo_check("127.0.0.1:30223", b"sombra30223");
        assert_eq!(s.delete(), Ok(()));
        if let Err(e) = res {
            panic!(format!("{:?}", e));
        }
    }

    #[test]
    fn spawn_once_delete_twice() {
        let s = match SombraLinux::build("tcp_echo", "executables/tcp_echo", vec![]) {
            Ok(s) => s,
            Err(e) => panic!(e.to_string()),
        };
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
        let s = match SombraLinux::build("tcp_echo", "executables/tcp_echo", vec![]) {
            Ok(s) => s,
            Err(e) => panic!(e.to_string()),
        };
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

// Run test on linux as sudo
// sudo -E cargo test -- --test-threads 1