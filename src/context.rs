use crate::prelude::*;
use futures_util::future::*;
use toml::*;

// pub struct Ref {
//     name :
// }

#[derive(Debug, Clone)]
pub enum Context {
    Workspace(Arc<WorkspaceContext>),
    Crate(Arc<CrateContext>),
}

impl Context {
    pub async fn load(location: &PathBuf) -> Result<Context> {
        let toml = async_std::fs::read_to_string(&location).await?;
        let tree: Value = toml::from_str(&toml)?;
        if tree.get("workspace").is_some() {
            Ok(Context::Workspace(Arc::new(
                WorkspaceContext::load(location).await?,
            )))
        } else {
            Ok(Context::Crate(Arc::new(
                CrateContext::load(location).await?,
            )))
        }
    }

    pub fn dependencies(&self) -> &Dependencies {
        match self {
            Context::Workspace(ctx) => &ctx.external,
            Context::Crate(ctx) => &ctx.dependencies,
        }
    }

    pub fn file(&self) -> &PathBuf {
        match self {
            Context::Workspace(ctx) => &ctx.file,
            Context::Crate(ctx) => &ctx.file,
        }
    }
}

#[derive(Debug)]
pub struct CrateContext {
    pub file: PathBuf,
    pub package: Package,
    pub dependencies: Dependencies,
}

impl CrateContext {
    pub async fn load(location: &PathBuf) -> Result<CrateContext> {
        let manifest = Crate::load(location).await?;

        Ok(CrateContext {
            file: manifest.file,
            package: manifest.package,
            dependencies: manifest.dependencies,
        })
    }
}

#[derive(Debug)]
pub struct WorkspaceContext {
    pub file: PathBuf,
    pub folder: PathBuf,
    pub manifest: Manifest,
    pub crates: Vec<Crate>,
    pub projects: Vec<String>,
    pub external: Dependencies,
}

impl WorkspaceContext {
    pub async fn load(location: &PathBuf) -> Result<WorkspaceContext> {
        let mut manifest = Manifest::load(location).await?;

        let folder = location.parent().unwrap_or_else(|| {
            panic!(
                "unable to determin parent folder for location: {}",
                location.display()
            )
        });

        let crates = manifest
            .workspace
            .members
            .iter()
            .map(|m| folder.join(m).join("Cargo.toml"))
            .collect::<Vec<_>>();

        let futures = crates.iter().map(Crate::load).collect::<Vec<_>>();

        let results = join_all(futures).await;

        let mut crates = results
            .into_iter()
            .enumerate()
            .map(|(idx, r)| {
                r.unwrap_or_else(|err| {
                    panic!("Error processing `{}`: {err}", crates[idx].display())
                })
            })
            .collect::<Vec<_>>();

        println!();
        let before = crates.len();
        crates.retain(|c| {
            let retain = c.package.publish.unwrap_or(true);
            if !retain {
                println!("...skipping {}", c.package.name)
            }
            retain
        });
        if before != crates.len() {
            println!();
        }

        let projects = crates
            .iter()
            .map(|crt| crt.package.name.clone())
            .collect::<Vec<_>>();

        let mut external = Dependencies::default();
        manifest.workspace.dependencies.retain(|name, dependency| {
            if projects.contains(name) {
                true
            } else {
                external.insert(name.clone(), dependency.clone());
                false
            }
        });

        crates
            .iter_mut()
            .for_each(|crt| crt.dependencies.retain(|name, _| projects.contains(name)));

        let mut publish_list = vec![];
        let mut publish_name_list = vec![];
        let length = crates.len();

        //should we use while ? maybe while can create infinite loop
        for _ in 0..length {
            if publish_list.len() == length {
                break;
            }
            for crt in crates.iter() {
                if publish_name_list.contains(&crt.name().to_string()) {
                    continue;
                }
                let mut deps = crt
                    .dependencies
                    .keys()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>();
                deps.retain(|dep| !publish_name_list.contains(dep));

                if deps.is_empty() {
                    publish_list.push(crt.clone());
                    publish_name_list.push(crt.name().to_string());
                }
            }
        }

        println!("=============");
        for crt in &publish_list {
            let deps = crt
                .dependencies
                .keys()
                .map(|c| c.to_string())
                .collect::<Vec<String>>();
            println!("{} -> {}", crt.name(), deps.join(", "));
        }

        Ok(WorkspaceContext {
            file: manifest.file.clone(),
            folder: folder.to_path_buf(),
            manifest,
            crates: publish_list,
            projects: publish_name_list,
            external,
        })
    }
}
