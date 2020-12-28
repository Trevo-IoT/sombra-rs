use structopt::StructOpt;
use sombra::{Sombra, SombraError};
use colored::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "sombra")]
enum CLIArgs {
    /// Create a service and start it
    Create {
        /// Name of service
        name: String,
        /// Path of service executable
        path: String,
    },
    /// Delete a service from system
    Delete {
        /// Name of service
        name: String
    },
}

fn main() -> Result<(), SombraError> {
    let args = CLIArgs::from_args();

    match args {
        CLIArgs::Create {name, path} => {
            sombra::build(&name, &path).create()?;
            println!("[{}] Service {} created with success", "OK".green(), name);
        },
        CLIArgs::Delete {name} => {
            sombra::build(&name, ".").delete()?;
            println!("[{}] Service {} deleted with success", "OK".green(), name);
        }
    }

    Ok(())
}
