use crate::prelude::*;
use crate::result::Result;
use clap::{Parser, Subcommand};

mod archive;
mod build;
mod check;
mod context;
mod crates;
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
        // setting = clap::AppSettings::DeriveDisplayOrder,
    )]
    Args(Args),
}

#[derive(Debug, clap::Args)]
struct Args {
    /// Workspace manifest location (Cargo.toml)
    #[clap(name = "manifest")]
    location: Option<String>,

    /// Action to execute
    #[clap(subcommand)]
    action: Action,

    /// Perform checks without actual processing.
    #[clap(short, long)]
    dry_run: bool,
}

#[derive(Subcommand, Debug)]
enum Action {
    // Test {},
    /// Update workspace version: 'major', 'minor', 'patch', 'x.y.z[-suffix]'
    Version { change: Change },
    /// Publish all crates in the workspace
    Publish,
    /// Check all dependency versions against those published on crates.io
    Check,
    /// Build WASM package targets specified within cargo's `package.metadata.emanate.wasm`
    Build {
        #[clap(short, long)]
        package: Option<Vec<String>>,
    },
}

pub async fn async_main() -> Result<()> {
    let args = Cmd::parse();
    let Cmd::Args(Args {
        action,
        location,
        dry_run,
    }) = args;
    let location = manifest::locate(location).await?;
    let ctx = Context::load(&location).await?;

    match action {
        // Action::Test {} => {
        //     println!("{ctx:#?}");
        //     println!("arg: {}", arg);
        //     let client = CratesIo::new_with_latency(500);
        //     let v = client.get_latest_version("base64").await?;
        //     println!("{}", v);
        // }
        Action::Version { change } => {
            let versioner = Versioner::new(ctx);
            versioner.change(change)?;
        }

        Action::Publish => {
            let publisher = Publisher::new(ctx);
            publisher.publish(dry_run).await?;
        }

        Action::Check => {
            let checker = Checker::new(ctx);
            checker.check().await?;
        }

        Action::Build { package } => {
            let builder = Builder::new(ctx);
            builder.build(package).await?;
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
