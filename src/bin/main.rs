extern crate nomoney;

use clap::{Arg, ArgAction, Command};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cmd = Command::new("NoMoney")
        .about("Family financial tracker api application")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("server")
                .about("Server management")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("start")
                        .required(false)
                        .long("start")
                        .help("Start api web server")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("stop")
                        .required(false)
                        .long("stop")
                        .help("Stop api web server")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("reload")
                        .required(false)
                        .long("reload")
                        .help("Reload api web server")
                        .action(ArgAction::SetTrue),
                ),
        )
        .get_matches();

    match cmd.subcommand() {
        Some(("server", sub_cmd)) => {
            if sub_cmd.get_one::<bool>("stop").unwrap().to_owned() {
                println!("Stopping server");
                Ok(())
            } else if sub_cmd.get_one::<bool>("reload").unwrap().to_owned() {
                println!("Reloading server");
                Ok(())
            } else {
                Ok(nomoney::cmd::server::run().await?)
            }
        }
        _ => Ok(()),
    }
}
