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

        let client = CratesIo::new_with_latency(300);
        let mut names = deps.keys().collect::<Vec<_>>();
        names.sort();
        let len = names.iter().map(|c| c.len()).fold(0, |a, b| a.max(b)) + 2;

        // pre-check versions
        for name in names.iter() {
            if let Err(err) = deps.get(*name).unwrap().version() {
                println!("`{name}`: {err}");
                let latest_version = client.get_latest_version(name).await?;
                println!("latest version for `{name}` is: `{latest_version}`");
                println!("aborting...");
                return Ok(());
            }
            // .map_err(|err| error!("Error processing dependency `{name}`: {err}"))?;
        }

        // check against crates.io
        for name in names.iter() {
            let dependency = deps.get(*name).unwrap();
            let version = dependency
                .version()
                .map_err(|err| error!("Error processing dependency `{name}`: {err}"))?;
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
