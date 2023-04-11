// use std::ptr::metadata;
use cfg_if::cfg_if;

use crate::prelude::*;
use convert_case::{Case, Casing};

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
                        let crate_name_snake = crate_name.to_case(Case::Snake);
                        // let crate_name_kebab = crate_name.to_case(Case::Kebab);
                        // TODO get version from crate and usee workspace as fallback
                        let version = manifest_version.clone();

                        let target_folder = ctx.folder.join("target/release");
                        let setup_folder = ctx.folder.join("setup");
                        // let setup_folder = crt.folder.join(&setup_folder);

                        log_info!("Build", "building {crate_name} @ {version}");

                        cfg_if! {
                            if #[cfg(target_arch = "aarch64")] {
                                let arch = "aarch64";
                            } else if #[cfg(target_arch = "x86_64")] {
                                let arch = "x64";
                            } else if #[cfg(target_arch = "arm")] {
                                let arch = "arm";
                            } else {
                                panic!("Unsupported architecture");
                            }
                        }

                        cfg_if! {
                            if #[cfg(target_os = "windows")] {
                                let platform = "win";
                            } else if #[cfg(target_os = "linux")] {
                                let platform = "linux";
                            } else if #[cfg(target_os = "macos")] {
                                let platform = "macos";
                            } else {
                                panic!("Unsupported platform");
                            }
                        }
                        if let Some(_build) = metadata.build.as_ref() {
                            cmd!("cargo", "build", "-p", &crate_name, "--release")
                                .dir(&ctx.folder)
                                .run()?;

                            cfg_if! {
                                if #[cfg(target_os = "windows")] {
                                    let extension = ".exe";
                                } else {
                                    let extension = "";
                                }
                            }

                            let archive_src = format!("{crate_name}{extension}");

                            let filename = format!("{crate_name}-{version}-{platform}-{arch}.zip");
                            // let setup_folder = build.folder.clone().unwrap_or("setup".to_string());
                            // let setup_folder = crt.folder.join(&setup_folder);
                            std::fs::create_dir_all(&setup_folder)?;
                            let archive_dest = setup_folder.join(filename);

                            if archive_dest.exists() {
                                log_info!("Build", "removing: `{}`", archive_dest.display());
                                std::fs::remove_file(&archive_dest)?;
                            }

                            // println!("--> {} {} {} {} {} {}","zip", "-9", archive_dest.display(), ".", "-i", archive_src);
                            cmd!("zip", "-9", &archive_dest, archive_src)
                                .dir(target_folder)
                                .run()?;

                            cmd!("du", "-h", &archive_dest).run()?;

                            log_info!("Build", "build complete: {crate_name} @ {version}");
                        }

                        if let Some(wasm) = metadata.wasm.as_ref() {
                            for wasm_target in wasm.targets.iter() {
                                let target = wasm_target.target.to_string();
                                let result = cmd!(
                                    "wasm-pack",
                                    "build",
                                    "--target",
                                    &target,
                                    "--out-dir",
                                    &wasm_target.out_dir
                                )
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
                                        // let setup_folder = wasm.folder.clone().unwrap_or("setup".to_string());
                                        // let setup_folder = crt.folder.join(&setup_folder);
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
                                            .dir(source_parent)
                                            .run()?;

                                        let main_file =
                                            source_folder.join(format!("{crate_name_snake}.js"));
                                        let docs_folder =
                                            wasm.docs.clone().unwrap_or("docs".to_string());
                                        let doc_dest = crt
                                            .folder
                                            .join(docs_folder)
                                            .join(format!("{crate_name}-{version}-{target}"));

                                        cmd!(
                                            "jsdoc",
                                            "--destination",
                                            doc_dest,
                                            main_file,
                                            "../README.md"
                                        )
                                        .dir(source_parent)
                                        .run()
                                        .map_err(|err| {
                                            log_error!("JsDoc", "error running jsdoc: {err}");
                                        })
                                        .ok();

                                        cmd!("du", "-h", &archive_dest).run()?;

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
