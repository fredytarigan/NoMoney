use anyhow::Result;
use clap::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("NoMoney")
        .about("Financial tracker for dummies!")
        .arg_required_else_help(true)
        .subcommand(Command::new("server").about("Start NoMoney api server"))
        .get_matches();

    match matches.subcommand() {
        Some(("server", _)) => Ok(cmd::server::run().await?),

        _ => Ok(()),
    }
}
