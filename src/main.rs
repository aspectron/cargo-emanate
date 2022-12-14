use std::collections::HashMap;

// use crate::config::*;
use crate::result::Result;
use clap::{Parser,Subcommand};
use duct::cmd;
use crate::prelude::*;
// use console::style;

mod error;
mod result;
mod config;
mod repository;
mod project;
mod build;
mod run;
mod log;
mod sync;
mod context;
mod prelude;
mod manifest;

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

    #[clap(name = "manifest")]
    location: Option<String>,

    #[clap(subcommand)]
    action : Action,

    #[clap(short, long)]
    verbose : bool,
}

#[derive(Subcommand, Debug)]
enum Action {
    Test,
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
    let Cmd::Args(Args { action, location, verbose }) = args;

    let ctx = Arc::new(Context::try_new(location)?);
    // let manifest = Manifest::load().await?;
    // let action = match args { Cmd::Args(args) => args.action };
    match action {
        Action::Test => {

            // println!("{:#?}",ctx);
            // println!("--------");
            let mut repositories = Vec::new();
            let mut projects = HashMap::new();
            for repository_config in ctx.manifest.project.repositories.iter() {
                let repository = Repository::try_from((&ctx,repository_config))?;
                repositories.push(repository.clone());

                if !repository.is_external() && repository.is_published() {
                    let project: Project = repository.try_into()?;
                    log_info!("Project","{} {}",project.name,style(&project.version).cyan());
                    for dep in project.dependencies.iter() {
                        log_info!(
                            "Dependency",
                            "\t{} {} {}",
                            dep.name,
                            style(dep.version.as_ref().map(|v|v.to_string()).unwrap_or("n/a".to_string())).cyan(),
                            style(dep.path.clone().unwrap_or("".to_string())).magenta());

                    }
                    // println!("{:#?}", project);
                    projects.insert(project.name.clone(), project);

                }

                println!("\n\n----------\n\n");

                let mut refs = Vec::new();
                for (_,project) in projects.iter() {
                    log_info!("Project","{} {}",project.name,style(&project.version).cyan());
                    let related = project.get_related()?;
                    for (name, path) in related.iter() {
                        if let Some(target) = projects.get(name) {
                            if &target.folder != path {
                                println!("WARNING: path mismatch for `{}`: `{}` vs `{}` ",
                                    name,
                                    style(path.clone().display()).cyan(),
                                    style(project.folder.display()).cyan()
                                );
                            }
                        } else {
                            log_info!("->","{} `{}`",name,path.display());
                            // refs.push((name.clone(),path.clone()));
                            let reference: Project = (&ctx,path.as_path()).try_into()?;
                            refs.push(reference);
                            // projects.insert(name.clone(),reference);
                        }
                    }
                    // for dep in project.dependencies.iter() {

                    // }
                }

                for project in refs {
                    projects.insert(project.name.clone(),project);
                }

                println!("\n\n----------\n\n");

                for (name, project) in projects.iter() {
                    log_info!("Project","{} {}",name,style(&project.version).cyan());
                } 

                // cargo_toml::Manifest::from_path(cargo_toml_path)
                // project.load_cargo_manifest()?;
                // let cargo_
                // projects.push(projects);
            }
            // println!("{:#?}",repos);
            
            // let list = ctx.manifest.project.repositories.iter().map(|r|Repository::from(r)).collect::<Vec<_>>();
            // for repo in ctx.manifest.project.repositories

        },
        Action::Sync => {

        },
        Action::Build => {

        },
        Action::Run => {
        },
        Action::Purge { force } => {

        },
        Action::Status => {

        },
        // Action::Sync => {
        //     for repository in config.repository.iter() {
        //         repository.sync().await?;
        //     }
        // },
        // Action::Build => {
        //     for build in config.build.expect("no build directives found").iter() {
        //         build.execute().await?;
        //     }
        // },
        // Action::Run => {
        //         let run = config.run.expect("no run directive found");
        //         run.execute().await?;
        // },
        // Action::Purge { force } => {
        //     match force {
        //         true => {
        //             for repository in config.repository.iter() {
        //                 println!("erasing: {}",repository.name());
        //                 cmd!("rm","-rf", repository.name()).run()?;
        //             }
        //         },
        //         false => {
        //             println!("--force flag is required for the purge operation");
        //         }
        //     }
        // },
        // Action::Status => {
        //     for repository in config.repository.iter() {
        //         println!("{}",repository.url);
        //     }
        // }
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