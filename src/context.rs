use crate::prelude::*;
use futures_util::future::*;

// pub struct Ref {
//     name :
// }

#[derive(Debug)]
pub struct Context {
    pub folder: PathBuf,
    pub manifest: Manifest,
    pub crates: Vec<Crate>,
    pub projects: Vec<String>,
    pub external: Vec<String>,
}

impl Context {
    pub async fn load(location: &PathBuf) -> Result<Context> {
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

        crates.retain(|c| {
            let retain = c.package.publish.unwrap_or(true);
            if !retain {
                println!("...skipping {}", c.package.name)
            }
            retain
        });

        let mut projects = crates
            .iter()
            .map(|crt| crt.package.name.clone())
            .collect::<Vec<_>>();

        let mut external = Vec::new();
        manifest.workspace.dependencies.retain(|name, _| {
            if projects.contains(name) {
                true
            } else {
                external.push(name.clone());
                false
            }
        });

        crates
            .iter_mut()
            .for_each(|crt| crt.dependencies.retain(|name, _| projects.contains(name)));

        crates.iter().for_each(|crt| {
            let name = &crt.package.name;
            let deps = crt.dependencies.keys().collect::<Vec<_>>();
            if deps.is_empty() {
                let value = projects.remove(projects.iter().position(|n| n == name).unwrap());
                projects.insert(0, value);
            } else {
                let project_name =
                    projects.remove(projects.iter().position(|n| n == name).unwrap());
                let mut pos = 0;
                deps.iter().for_each(|dep_name| {
                    pos = std::cmp::max(
                        pos,
                        projects
                            .iter()
                            .position(|project| project == *dep_name)
                            .unwrap()
                            + 1,
                    );
                });
                projects.insert(pos, project_name);
            }
        });

        Ok(Context {
            folder: folder.to_path_buf(),
            manifest,
            crates,
            projects,
            external,
        })
    }
}
