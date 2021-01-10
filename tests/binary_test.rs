
#[derive(Debug, PartialEq)]
enum CommandOutput {
    Stdout(String),
    Stderr(String, i32),
}

macro_rules! stdout {
    ($msg:expr) => {CommandOutput::Stdout($msg.to_string())};
}

fn run_cmd(cmd: &str, args: Vec<&str>) -> std::io::Result<CommandOutput> {
    let output = std::process::Command::new(cmd)
        .args(args)
        .output()?;

    let code = output.status.code().unwrap();
    if code == 0 {
        let stdout = std::str::from_utf8(output.stdout.as_slice()).unwrap();
        Ok(CommandOutput::Stdout(stdout.to_string()))
    } else {
        let stderr = std::str::from_utf8(output.stderr.as_slice()).unwrap();
        Ok(CommandOutput::Stderr(stderr.to_string(), code))
    }
}

#[test]
#[cfg(target_os = "linux")]
fn linux_normal_flow() -> std::io::Result<()> {
    let res = run_cmd("./target/release/sombra", vec!["create", "tcp_echo",
                                            "executables/tcp_echo"])?;
    assert_eq!(res, stdout!("[OK] Service tcp_echo created with success\n"));

    let res = run_cmd("./target/release/sombra", vec!["delete",
                                                      "tcp_echo"])?;
    assert_eq!(res, stdout!("[OK] Service tcp_echo deleted with success\n"));

    Ok(())
}

#[test]
#[cfg(target_os = "linux")]
fn linux_error_flow() -> std::io::Result<()>  {
    run_cmd("./target/release/sombra", vec!["create", "tcp_echo", "executables/tcp_echo"])?;

    let already_exist = run_cmd("./target/release/sombra", vec!["create", "tcp_echo", "executables/tcp_echo"])?;
    assert_eq!(already_exist, stdout!("[ERR] <Io> Service tcp_echo already exist\n"));

    let file_not_found = run_cmd("./target/release/sombra", vec!["create", "tcp_echo2", "executables/tcp_echos"])?;
    assert_eq!(file_not_found, stdout!("[ERR] <Io> executables/tcp_echos: No such file or directory (os error 2)\n"));

    run_cmd("./target/release/sombra", vec!["delete", "tcp_echo"])?;

    let service_not_find = run_cmd("./target/release/sombra", vec!["delete", "tcp_echo2"])?;
    assert_eq!(service_not_find, stdout!("[ERR] <Io> No such file or directory (os error 2)\n"));

    Ok(())
}
