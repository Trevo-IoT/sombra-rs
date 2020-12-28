use structopt::StructOpt;
use std::net::TcpStream;
use std::io::{Write, Read};
use sombra::Sombra;

#[derive(StructOpt, Debug)]
#[structopt(name = "sombra")]
struct CLIArgs {
    #[structopt(short, long)]
    remove: bool,
}

//  sombra ser.exe [FLAGS] [OPTIONS]
//  [FLAGS]
//      remove
//

fn main() {
    let args = CLIArgs::from_args();
    println!("{:#?}", args);

    let _s = sombra::build("tcp_echo", "executables/tcp_echo.exe");
}
