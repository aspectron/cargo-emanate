use crate::prelude::*;

/// Checks for the latest version of the crate
#[derive(Debug)]
pub struct Checker {
    ctx: Context,
}

impl Checker {
    pub fn new(ctx: Context) -> Checker {
        Checker { ctx }
    }

    pub async fn check(&self) -> Result<()> {
        let deps = self.ctx.dependencies();

        let client = CratesIo::new();
        let mut names = deps.keys().collect::<Vec<_>>();
        names.sort();
        let len = names.iter().map(|c| c.len()).fold(0, |a, b| a.max(b)) + 2;

        // pre-check versions
        for name in names.iter() {
            match deps.get(*name).unwrap().version() {
                Err(Error::WorkspaceCrate) => {
                    println!(
                        "{} `{}`",
                        style("this appears to be a workspace crate:").red(),
                        style(self.ctx.file().display()).yellow()
                    );
                    return Ok(());
                }
                Err(Error::RelativeCrate) => {
                    println!("`{name}`: relative crate, ignoring...");
                }
                Err(err) => {
                    println!("`{name}`: {err}");
                    let latest_version = client.get_latest_version(name).await?;
                    println!("latest version for `{name}` is: `{latest_version}`");
                    println!("aborting...");
                    return Ok(());
                }
                _ => {}
            }
            // if let Err(err) = deps.get(*name).unwrap().version() {
            //     if matches!(err,Error::WorkspaceCrate) {
            //     }
            // }
            // .map_err(|err| error!("Error processing dependency `{name}`: {err}"))?;
        }

        // check against crates.io
        for name in names.iter() {
            let dependency = deps.get(*name).unwrap();
            let version = match dependency.version() {
                Ok(v) => v,
                Err(_) => continue,
            };
            // .map_err(|err| error!("Error processing dependency `{name}`: {err}"))?;
            let latest_version = client.get_latest_version(name).await?;
            if version != latest_version {
                println!(
                    "{}",
                    style(format!(
                        "{:>4} {version} -> {latest_version} - update",
                        name.pad(len, ' ', Alignment::Right, false)
                    ))
                    .yellow()
                );
            } else {
                println!(
                    "{}",
                    style(format!(
                        "{:>4} {version} -- {latest_version} - ok",
                        name.pad(len, ' ', Alignment::Right, false)
                    ))
                    .green()
                );
            }
        }

        Ok(())
    }
}
