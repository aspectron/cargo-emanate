use crate::manifest::*;
use crate::result::Result;
use clap::{Parser,Subcommand};
use duct::cmd;
// use console::style;

mod error;
mod result;
mod manifest;
mod repository;
mod build;
mod run;

#[derive(Debug, Parser)]
#[clap(name = "cargo")]
#[clap(bin_name = "cargo")]
#[clap(
    // setting = AppSettings::SubcommandRequiredElseHelp,
    setting = clap::AppSettings::DeriveDisplayOrder,
    dont_collapse_args_in_usage = true,
)]
enum Cmd {
    #[clap(name = "emanate")]
    #[clap(about, author, version)]
    #[clap(
        setting = clap::AppSettings::DeriveDisplayOrder,
    )]
    Args(Args),
}


#[derive(Debug, clap::Args)]
struct Args {
    #[clap(subcommand)]
    action : Action,
    // #[clap(short, long)]
    // verbose : Option<bool>,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// List status of all repositories
    Status,
    /// Sync (Clone or Pull) repositories listed in the manifest
    Sync,
    /// Build the `[build]` entries listed in the manifest
    Build,
    /// Run the `[run]` entry in the manifest
    Run,
    /// Purge repositories liste in the manifest (requires force parameter)
    Purge { 
        #[clap(short, long)]
        force : bool
    },
}


pub async fn async_main() -> Result<()> {
    
    // let cwd = std::env::current_dir()?;
    let args = Cmd::parse();
    let manifest = Manifest::load().await?;
    let action = match args { Cmd::Args(args) => args.action };
    match action {
        Action::Sync => {
            for repository in manifest.repository.iter() {
                repository.sync().await?;
            }
        },
        Action::Build => {
            for build in manifest.build.expect("no build directives found").iter() {
                build.execute().await?;
            }
        },
        Action::Run => {
                let run = manifest.run.expect("no run directive found");
                run.execute().await?;
        },
        Action::Purge { force } => {
            match force {
                true => {
                    for repository in manifest.repository.iter() {
                        println!("erasing: {}",repository.name());
                        cmd!("rm","-rf", repository.name()).run()?;
                    }
                },
                false => {
                    println!("--force flag is required for the purge operation");
                }
            }
        },
        Action::Status => {
            for repository in manifest.repository.iter() {
                println!("{}",repository.url);
            }
        }
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