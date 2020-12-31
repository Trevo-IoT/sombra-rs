use structopt::clap::AppSettings;
use structopt::StructOpt;
use sombra::Sombra;
use colored::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "sombra")]
enum CLIArgs {
    /// Create a service and start it
    #[structopt(setting = AppSettings::AllowLeadingHyphen)]
    Create {
        /// Name of service
        name: String,
        /// Path of service executable
        path: String,
        /// Arguments of target process
        args: Vec<String>,
    },
    /// Delete a service from system
    Delete {
        /// Name of service
        name: String
    },
}

fn main() -> sombra::Result<()> {
    let args = CLIArgs::from_args();

    match args {
        CLIArgs::Create {name, path, mut args } => {
            args.retain(|x| x != "");
            sombra::build(&name, &path, args).create()?;
            println!("[{}] Service {} created with success", "OK".green(), name);
        },
        CLIArgs::Delete {name} => {
            sombra::build(&name, ".", vec![]).delete()?;
            println!("[{}] Service {} deleted with success", "OK".green(), name);
        }
    }

    Ok(())
}
