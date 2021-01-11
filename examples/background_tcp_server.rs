use sombra::Sombra;

use std::net::TcpStream;
use std::io::{Write, Read};

fn main() -> sombra::Result<()> {
    // Path of TCP Server executable
    let executable_path = if cfg!(target_os = "windows") {
        "executables\\tcp_echo.exe"
    } else {
        "executables/tcp_echo"
    };
    // Buffer to handle incoming TCP Server response
    let mut buffer = [0u8; 512];
    // Message to send to Background TCP Server
    let msg = "Hello sombra";

    // Creating sombra obj
    let tcp_server = sombra::build("tcp_server", executable_path, vec![])?;
    // Creating and starting a service, in this case, a backgrounding TCP Server
    tcp_server.create()?;

    // Connecting to Backgrounding TCP Server at localhost (127.0.0.1), in the port 30222
    let mut stream = TcpStream::connect("127.0.0.1:30222")?;
    println!("Sending \"{}\" to Background TCP Server...", msg);
    // Sending a message to the backgrounding TCP Server
    stream.write(msg.as_bytes())?;
    println!("Message sent with success.");

    // Wait the response of backgrounding TCP Server
    stream.read(&mut buffer)?;
    // Cast 'buffer' to Vector
    let mut buffer = buffer.to_vec();
    // Retain only non empty bytes
    buffer.retain(|&x| x != 0);
    println!("Receive from Background TCP Server: {}", std::str::from_utf8(buffer.as_slice()).unwrap());

    println!("Deleting Background TCP Server...");
    // Stopping and Removing service (The backgrounding TCP Server) created
    tcp_server.delete()?;
    println!("Background TCP Server deleted.");

    Ok(())
}
