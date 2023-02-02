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

        crates.sort_by(|a, b| {
            use std::cmp::Ordering;
            let name_a = a.name();
            let name_b = b.name();
            // let deps_a = a.dependencies.get(b);
            if a.dependencies.get(name_b).is_some() {
                Ordering::Greater
            } else if b.dependencies.get(name_a).is_some() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        for crt in crates.iter() {
            let deps = crt.dependencies.keys().map(|c|c.to_string()).collect::<Vec<_>>().join(", ");
            println!("{} -> {deps}", crt.name());
        }

panic!();

        // let projects = crates
        //     .iter()
        //     .map(|crt| crt.package.name.clone())
        //     .collect::<Vec<_>>();

        // crates.iter().for_each(|crt| {
        //     let name = &crt.package.name;
        //     let deps = crt.dependencies.keys().collect::<Vec<_>>();
        //     if deps.is_empty() {
        //         let value = projects.remove(projects.iter().position(|n| n == name).unwrap());
        //         projects.insert(0, value);
        //     } else {

        //         let project_name =
        //             projects.remove(projects.iter().position(|n| n == name).unwrap());
        //         println!("removing project {project_name}");
        //         println!("{projects:#?}");
        //         let mut pos = 0;
        //         deps.iter().for_each(|dep_name| {
        //             println!("{project_name} -> {dep_name}");
        //             pos = std::cmp::max(
        //                 pos,
        //                 projects
        //                     .iter()
        //                     .position(|project| project == *dep_name)
        //                     .unwrap()
        //                     + 1,
        //             );
        //         });
        //         println!("position: {pos}");
        //         projects.insert(pos, project_name);
        //         println!("{projects:#?}");

        //     }
        // });
        // for project in projects.iter() {

        // }

        // for crt in crates.iter() {
        //     let deps = crt.dependencies.keys().map(|c|c.to_string()).collect::<Vec<_>>().join(", ");
        //     println!("{} -> {deps}", crt.name());
        // }

        // println!("{:#?}", projects);
        panic!();
        Ok(WorkspaceContext {
            file: manifest.file.clone(),
            folder: folder.to_path_buf(),
            manifest,
            crates,
            projects,
            external,
        })
    }
}
