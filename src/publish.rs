use crate::prelude::*;

pub struct Publisher {
    ctx: Arc<Context>,
}

impl Publisher {
    pub fn new(ctx: &Arc<Context>) -> Self {
        Self { ctx: ctx.clone() }
    }

    pub async fn publish(&self) -> Result<()> {
        let crates_io = CratesIo::new();
        let manifest_version = self.ctx.manifest.version()?;

        for project in self.ctx.projects.iter() {
            let version = crates_io.get_latest_version(project).await?;

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
