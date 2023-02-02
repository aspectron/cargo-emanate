use crate::prelude::*;

/// Checks for the latest version of the crate
#[derive(Debug)]
pub struct Checker {
    ctx: Arc<Context>,
}

impl Checker {
    pub fn new(ctx: &Arc<Context>) -> Checker {
        Checker { ctx: ctx.clone() }
    }

    pub async fn check(&self) -> Result<()> {
        let client = CratesIo::new_with_latency(500);
        let mut names = self.ctx.external.keys().collect::<Vec<_>>();
        names.sort();
        let len = names.iter().map(|c| c.len()).fold(0, |a, b| a.max(b)) + 2;

        for name in names {
            let dependency = self.ctx.external.get(name).unwrap();
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
