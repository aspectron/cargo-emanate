use convert_case::{Case, Casing};
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
                                let result = cmd!("wasm-pack", "build", "--target", &target, "--out-dir", &wasm_target.out_dir)
                                    .dir(&crt.folder)
                                    .run();
                                // let result = Result::Ok(());
                                match result {
                                    Ok(_) => {
                                        // zip -r filename.zip source-folder/   or   tar -pvczf filename.tar.gz /path/to/directory
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
                                        let setup_folder = wasm.folder.clone().unwrap_or("setup".to_string());
                                        let setup_folder = crt.folder.join(&setup_folder);
                                        std::fs::create_dir_all(&setup_folder)?;
                                        let archive_dest = setup_folder.join(filename);

                                        if archive_dest.exists() {
                                            log_info!(
                                                "Build",
                                                "removing: `{}`",
                                                archive_dest.display()
                                            );
                                            std::fs::remove_file(&archive_dest)?;
                                        }

                                        cmd!("zip", "-r", "-9", &archive_dest, archive_folder)
                                            .dir(&source_parent)
                                            .run()?;
                                        
                                        let snake_crate_name = crate_name.to_case(Case::Snake);
                                        let main_file = source_folder.join(format!("{snake_crate_name}.js"));
                                        let docs_folder = wasm.docs.clone().unwrap_or("docs".to_string());
                                        let doc_dest = crt.folder.join(docs_folder).join(format!("{crate_name}-{version}-{target}"));
                                        
                                        cmd!("jsdoc", "--destination", doc_dest, main_file, "../README.md")
                                            .dir(&source_parent)
                                            .run()
                                            .map_err(|err| {
                                                log_error!("JsDoc","error running jsdoc: {err}");
                                            }).ok();

                                        cmd!("du", "-h", &archive_dest)
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
