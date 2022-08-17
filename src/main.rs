use crate::manifest::*;
use crate::result::Result;
use clap::{Parser,Subcommand,AppSettings};
use duct::cmd;

mod error;
mod result;
mod manifest;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp)]
struct Args {
    #[clap(subcommand)]
    action : Action,
    // #[clap(short, long)]
    // verbose : Option<bool>,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Clone repositories listed in the manifest
    Clone,
    /// Purge repositories liste in the manifest (requires force parameter)
    Purge { 
        #[clap(short, long)]
        force : bool
    },
}

pub async fn async_main() -> Result<()> {

    let args = Args::parse();
    let manifest = Manifest::load().await?;

    match args.action {
        Action::Clone => {
            for repository in manifest.repository.iter() {
                cmd!("git","clone", &repository.url).run()?;
            }
        },
        Action::Purge { force } => {
            match force {
                true => {
                    for repository in manifest.repository.iter() {
                        cmd!("rm -rf", repository.name()).run()?;
                    }
                },
                false => {
                    println!("--force flag is required for the purge operation");
                }
            }
        },
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    match async_main().await {
        Err(e) => println!("{}", e),
        Ok(_) => { }
    };
    Ok(())
}