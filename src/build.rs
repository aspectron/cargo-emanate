use crate::prelude::*;

pub struct Builder {
    ctx: Context,
}

impl Builder {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }

    pub async fn build(&self) -> Result<()> {
        match &self.ctx {
            Context::Workspace(ctx) => {
                // let crates_io = CratesIo::new();
                let manifest_version = ctx.manifest.version()?;

                for crt in ctx.crates.iter() {
                    if let Some(metadata) = crt.metadata()?.as_ref() {
                        let crate_name = crt.package.name.clone();
                        // TODO get version from crate and usee workspace as fallback
                        let version = manifest_version.clone();
                        log_info!("Build", "building {crate_name} @ {version}");

                        if let Some(wasm) = metadata.wasm.as_ref() {
                            for wasm_target in wasm.targets.iter() {
                                let target = wasm_target.target.to_string();
                                // let out_dir = serde_json::to_string(&wasm_target.out_dir)?;
                                // let result = cmd!("wasm-pack", "build", "--target", &target, "--out-dir", &wasm_target.out_dir)
                                //     .dir(&crt.folder)
                                //     .run();
                                let result = Result::Ok(());
                                match result {
                                    Ok(_) => {
                                        // zip -r mynewfilename.zip foldertozip/   or   tar -pvczf BackUpDirectory.tar.gz /path/to/directory
                                        let out_dir = PathBuf::from(&wasm_target.out_dir);
                                        let source_folder = crt.folder.join(&out_dir);
                                        let source_parent = source_folder.parent().unwrap_or_else(||panic!("unable to get parent directory from `out-dir`: '{}'",wasm_target.out_dir));
                                        let archive_folder = out_dir
                                            .file_name()
                                            .unwrap_or_else(|| {
                                                panic!(
                                                    "unable to get file name from `out-dir`: '{}'",
                                                    wasm_target.out_dir
                                                )
                                            })
                                            .to_str()
                                            .unwrap()
                                            .to_string();
                                        let filename =
                                            format!("{crate_name}-{version}-{target}.zip");
                                        let destination_folder = crt.folder.join(&wasm.folder);
                                        std::fs::create_dir_all(&destination_folder)?;
                                        let destination = destination_folder.join(filename);

                                        if destination.exists() {
                                            log_info!(
                                                "Build",
                                                "removing: `{}`",
                                                destination.display()
                                            );
                                            std::fs::remove_file(&destination)?;
                                        }

                                        cmd!("zip", "-r", "-9", &destination, archive_folder)
                                            .dir(&source_parent)
                                            .run()?;

                                        // println!("running in dest folder: {}", destination_folder.display());
                                        cmd!("du", "-h", &destination)
                                            // .dir(&destination_folder)
                                            .run()?;

                                        log_info!(
                                            "Build",
                                            "build complete: {crate_name} @ {version}"
                                        );
                                    }
                                    Err(err) => {
                                        println!("\n{err}\n");
                                        println!("\t->  {crate_name}\n");
                                        return Ok(());
                                    }
                                }
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

// pub struct WasmBuilder {
//     ctx: Context,
// }

// impl WasmBuilder {
//     pub fn new(ctx: Context) -> Self {
//         Self { ctx }
//     }

//     pub fn build(&self, target : WasmTarget) -> Result<()> {
//     }
// }
