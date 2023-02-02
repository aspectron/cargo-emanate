use crate::prelude::*;

pub struct Publisher {
    ctx: Context,
}

impl Publisher {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }

    pub async fn publish(&self) -> Result<()> {
        match &self.ctx {
            Context::Workspace(ctx) => {
                let crates_io = CratesIo::new();
                let manifest_version = ctx.manifest.version()?;

                for project in ctx.projects.iter() {
                    let version = crates_io.get_latest_version(project).await?;

                    if version == manifest_version {
                        log_info!("Skipping", "{project} {manifest_version} -> {version}");
                    } else {
                        log_info!("Publishing", "{project} {manifest_version} -> {version}");
                        let result = cmd!("cargo", "publish", "--package", project)
                            .dir(&ctx.folder)
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
            }
            _ => {
                panic!("not currently supported in the context of a single crate");
            }
        }

        Ok(())
    }
}
