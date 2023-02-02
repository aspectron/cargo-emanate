use crate::prelude::*;
use crate::result::Result;
use clap::{Parser, Subcommand};

mod context;
mod error;
mod log;
mod manifest;
mod prelude;
mod publish;
mod result;
mod utils;
mod version;

#[derive(Debug, Parser)]
#[clap(name = "cargo")]
#[clap(bin_name = "cargo")]
#[clap(
    // setting = AppSettings::SubcommandRequiredElseHelp,
    // setting = clap::AppSettings::DeriveDisplayOrder,
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
    #[clap(name = "manifest")]
    location: Option<String>,

    #[clap(subcommand)]
    action: Action,
    // #[clap(short, long)]
    // verbose : bool,
}

#[derive(Subcommand, Debug)]
enum Action {
    Test {
        // arg : String,
    },
    Version {
        /// Update workspace version: 'major', 'minor', 'patch', 'x.y.z'
        change: Change,
    },
    Publish,
    // Purge {
    //     #[clap(short, long)]
    //     force : bool
    // },
}

pub async fn async_main() -> Result<()> {
    // let cwd = std::env::current_dir()?;
    let args = Cmd::parse();
    let Cmd::Args(Args { action, location }) = args;

    let location = Manifest::locate(location).await?;
    let ctx = Arc::new(Context::load(&location).await?);

    match action {
        Action::Test {} => {
            // println!("{ctx:#?}");
            // println!("arg: {}", arg);
        }
        Action::Version { change } => {
            let versioner = Versioner::new(&ctx);
            versioner.change(change)?;
        }

        Action::Publish => {
            let publisher = Publisher::new(&ctx);
            publisher.publish().await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(err) = async_main().await {
        println!("{err}")
    }
    Ok(())
}
