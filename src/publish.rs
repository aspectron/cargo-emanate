use crate::prelude::*;

pub struct Publisher {
    ctx: Arc<Context>,
}

impl Publisher {
    pub fn new(ctx: &Arc<Context>) -> Self {
        Self { ctx: ctx.clone() }
    }

    pub async fn publish(&self) -> Result<()> {
        let client = crates_io_api::AsyncClient::new(
            "cargo-emanate (info@aspectron.com)",
            std::time::Duration::from_millis(1000),
        )
        .unwrap();

        let manifest_version = self.ctx.manifest.version()?;

        for project in self.ctx.projects.iter() {
            let crt = client.get_crate(project).await?;
            let mut versions = crt
                .versions
                .iter()
                .map(|v| {
                    v.num.parse::<Version>().unwrap_or_else(|err| {
                        panic!(
                            "Unable to parse version for crate `{project}` - `{}`: {err}",
                            v.num
                        );
                    })
                })
                .collect::<Vec<_>>();
            versions.sort_by(|a, b| b.cmp(a));
            let version = versions
                .first()
                .unwrap_or_else(|| panic!("No versions present for crate {project}"))
                .to_owned();

            if version == manifest_version {
                log_info!("Skipping", "{project} {manifest_version} -> {version}");
            } else {
                log_info!("Publishing", "{project} {manifest_version} -> {version}");
                let result = cmd!("cargo", "publish", "--package", project)
                    .dir(&self.ctx.folder)
                    .run();
                match result {
                    Ok(_) => {
                        log_info!("Success", "published {project} @ {version}");
                    }
                    Err(err) => {
                        println!("\n{err}\n");
                        println!("\t->  {project}\n");
                        return Ok(());
                    }
                }
            }
        }

        Ok(())
    }
}
