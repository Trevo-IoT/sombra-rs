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

fn cli_handler(args: CLIArgs) -> sombra::Result<String> {
    let success_msg = match args {
        CLIArgs::Create {name, path, mut args } => {
            args.retain(|x| x != "");
            sombra::build(&name, &path, args)?.create()?;
            format!("Service {} created with success", name)
        },
        CLIArgs::Delete {name} => {
            sombra::build(&name, ".", vec![])?.delete()?;
            format!("Service {} deleted with success", name)
        }
    };

    Ok(success_msg)
}

fn main() {
    let args = CLIArgs::from_args();

    match cli_handler(args) {
        Ok(success_msg) => println!("[{}] {}", "OK".green(), success_msg),
        Err(e) => println!("[{}] {}", "ERR".red(), e.to_string()),
    }
}
