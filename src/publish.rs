use crate::prelude::*;

pub struct Publisher {
    ctx: Context,
}

impl Publisher {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }

    pub async fn publish(&self, dry_run: bool) -> Result<()> {
        match &self.ctx {
            Context::Workspace(ctx) => {
                let crates_io = CratesIo::new();
                let manifest_version = ctx.manifest.version()?;

                let mut new_publish_list = HashMap::new();

                for crt in ctx.crates.iter() {
                    let project = &crt.name().to_string();
                    let version = crates_io.get_latest_version(project).await?;

                    if version == manifest_version {
                        log_info!("Skipping", "{project} {manifest_version} -> {version}");
                    } else {
                        log_info!("Publishing", "{project} {manifest_version} -> {version}");
                        if dry_run {
                            if crt.dependencies.is_empty() {
                                let key = format!("{project}/{manifest_version}");
                                new_publish_list.insert(key, version.to_string());
                                continue;
                            }
                            log_info!("Dependencies", "");
                            for (dep, dep_info) in &crt.dependencies {
                                let version = match dep_info.version() {
                                    Ok(v) => Some(v.to_string()),
                                    Err(e) => {
                                        match e {
                                            Error::WorkspaceCrate => {
                                                //Some("TODO".to_string())
                                                if let Some(info) =
                                                    ctx.manifest.workspace.dependencies.get(dep)
                                                {
                                                    if let Ok(v) = info.version() {
                                                        Some(v.to_string())
                                                    } else {
                                                        None
                                                    }
                                                } else {
                                                    None
                                                }
                                            }
                                            _ => None,
                                        }
                                    }
                                };

                                if let Some(v) = version {
                                    let key = format!("{dep}/{v}");
                                    if !new_publish_list.contains_key(&key) {
                                        log_error!("Error", "{dep} => unable to find {dep}/{v}");
                                        // let version =
                                        //     crates_io.get_latest_version(dep).await?.to_string();
                                        // if version != v {
                                        //     // TODO version compare
                                        //     log_error!("Error", "{dep} => unable to find version from crates_io ({v}!={version})");
                                        // } else {
                                        //     log_info!("", "{dep} => {v}");
                                        // }
                                    } else {
                                        log_info!("", "{dep} => {v}");
                                    }
                                } else {
                                    log_error!("Error", "{dep} => unable to get version");
                                }
                            }

                            let key = format!("{project}/{manifest_version}");
                            new_publish_list.insert(key, version.to_string());

                            continue;
                        }

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
