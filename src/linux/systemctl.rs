pub struct Systemctl {
    name: String
}

impl Systemctl {
    pub fn new(name: &str) -> Self {
        Systemctl {
            name: name.to_string()
        }
    }

    pub fn start(&self) -> crate::Result<()> {
        let _ = std::process::Command::new("systemctl")
            .arg("start")
            .arg(&self.name)
            .output()?;
        Ok(())
    }

    pub fn stop(&self) -> crate::Result<()> {
        let _ = std::process::Command::new("systemctl")
            .arg("stop")
            .arg(&self.name)
            .output()?;
        Ok(())
    }

    pub fn _is_active(&self) -> crate::Result<bool> {
        let output = std::process::Command::new("systemctl")
            .arg("is-active")
            .arg(&self.name)
            .output()?;
        Ok(std::str::from_utf8(output.stdout.as_slice())? == "active")
    }

    pub fn disable(&self) -> crate::Result<()> {
        let _ = std::process::Command::new("systemctl")
            .arg("disable")
            .arg(&self.name)
            .output()?;
        Ok(())
    }

    pub fn daemon_reload() -> crate::Result<()> {
        let _ = std::process::Command::new("systemctl")
            .arg("daemon-reload")
            .output()?;
        Ok(())
    }

    pub fn reset_failed() -> crate::Result<()> {
        let _ = std::process::Command::new("systemctl")
            .arg("reset-failed")
            .output()?;
        Ok(())
    }
}
