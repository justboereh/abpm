use clap::builder::{Arg, Command};
use clap::{command, ArgAction};

mod installer;

#[tokio::main]
async fn main() {
    let install_command = Command::new("install").about("Install packages").args([
        Arg::new("dev")
            .long("dev")
            .short('D')
            .help("Install as devDependencies"),
        Arg::new("offline")
            .long("offline")
            .short('O')
            .help("Install only from offline repository"),
        Arg::new("packages")
            .action(ArgAction::Append)
            .help("Packages"),
    ]);

    let matches = command!()
        .about("A blazingly bad package manager for Node.")
        .version("0.0.1")
        .subcommand(install_command)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("install") {
        let _ = installer::install(&matches).await;
    }
}
