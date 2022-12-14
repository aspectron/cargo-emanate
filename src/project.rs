use std::str::FromStr;

use crate::prelude::*;
use cargo_toml::{Manifest as CargoManifest,Dependency as CargoDependency, Inheritable, Package, DependencyDetail};


#[derive(Debug, Clone)]
pub struct Version {
    pub wildcard : bool,
    pub major: Option<u32>,
    pub minor: Option<u32>,
    pub patch: Option<u32>,
    pub suffix: Option<String>,
}

impl Version {
    pub fn get(&self) -> [u32;3] {
        [
            self.major.unwrap_or(0),
            self.minor.unwrap_or(0),
            self.patch.unwrap_or(0)
        ]
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.get();
        write!(f, "{}.{}.{}",v[0],v[1],v[2])
    }
}


impl FromStr for Version {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s == "*" {
            Ok(Version {
                wildcard: true,
                major: None,
                minor: None,
                patch: None,
                suffix: None,
            })
        } else { 
            let mut parts = s.split('.');
            let major = if let Some(v) = parts.next() { v.parse().ok() } else { None };
            let minor = if let Some(v) = parts.next() { v.parse().ok() } else { None };
            let patch = if let Some(v) = parts.next() { v.parse().ok() } else { None };
            let suffix = match parts.next() {
                Some(s) => Some(s.to_string()),
                None => None,
            };

            Ok(Version { major, minor, patch, suffix, wildcard : false })
        }

    }
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name : String,
    pub version : Option<Version>,
    pub path: Option<String>,
}

impl TryFrom<(&str,&str)> for Dependency {
    type Error = Error;
    fn try_from((name,version): (&str,&str)) -> Result<Self> {
        Ok(Dependency {
            name: name.to_string(),
            version: Some(Version::from_str(version)?),
            path: None,
        })
    }
}

impl TryFrom<(&str,&DependencyDetail)> for Dependency {
    type Error = Error;
    fn try_from((name,detail): (&str,&DependencyDetail)) -> Result<Self> {
        let version = if let Some(version) = &detail.version {
            Some(Version::from_str(&version)?)
        } else { None };
        Ok(Dependency {
            name: name.to_string(),
            version, //: detail.version.map(|v|Version::from_s) Some(Version::from_str(detail.version)?),
            path: detail.path.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    pub ctx : Arc<Context>,
    pub name: String,
    pub folder : PathBuf,
    pub repository : Option<Repository>,
    pub version : Version,
    pub dependencies : Vec<Dependency>,
    pub url : Option<String>,
    pub cargo_toml_file: PathBuf,
}

impl TryFrom<Repository> for Project {
    type Error = Error;
    fn try_from(repo: Repository) -> Result<Self> {
        let manifest = Manifest::try_load(&repo.folder)?;

        if repo.name != manifest.name {
            return Err(error!("manifest and repository name mismatch: `{}` vs `{}`", repo.name, manifest.name));
        }

        let project = Project {
            ctx : repo.ctx.clone(),
            name : repo.name.clone(),
            folder : repo.folder.clone(),
            repository : Some(repo.clone()),
            cargo_toml_file : manifest.cargo_toml_file,
            url : manifest.url,
            version : manifest.version,
            dependencies : manifest.dependencies,
        };

        Ok(project)
    }
}

impl TryFrom<(&Arc<Context>,&Path)> for Project {
    type Error = Error;
    fn try_from((ctx,folder): (&Arc<Context>,&Path)) -> Result<Self> {
        let manifest = Manifest::try_load(&folder)?;

        let project = Project {
            ctx : ctx.clone(),
            name : manifest.name.clone(),
            folder : folder.to_path_buf(),
            repository : None, //repo.clone(),
            cargo_toml_file : manifest.cargo_toml_file,
            url : manifest.url,
            version : manifest.version,
            dependencies : manifest.dependencies,
        };

        Ok(project)
    }
}

impl Project {
    pub fn get_related(&self) -> Result<Vec<(String,PathBuf)>> {
        let mut list = Vec::new();
        for dep in self.dependencies.iter() {
            if let Some(path) = dep.path.as_ref() {
                let path = self.folder.join(path).canonicalize()?;
                list.push((dep.name.clone(), path.clone()));
            }
        }

        Ok(list)
    }
}

#[derive(Debug, Clone)]
pub struct Manifest {
    pub name : String,
    pub cargo_toml_file : PathBuf,
    pub version : Version,
    pub dependencies : Vec<Dependency>,
    pub url : Option<String>,
}

impl Manifest {
    pub fn try_load(folder: &Path) -> Result<Manifest> {

        // if self.external {
        //     return Ok(());
        // }

        let cargo_toml_file = folder.join("Cargo.toml");
        // let manifest = Manifest::load(&manifest_path)?;

        let manifest = match CargoManifest::from_path(&cargo_toml_file) {
            Ok(manifest) => manifest,
            Err(err) => {
                return Err(error!("Unable to process `{}`: {}",cargo_toml_file.display(),err));
            }
        };

        // log_info!("Cargo.toml","at `{}`",cargo_toml_path.display());

        match Manifest::digest(&cargo_toml_file, &manifest) {
            Ok(manifest) => Ok(manifest),
            Err(err) => {
                return Err(error!("Unable to digest manifest at `{}`: {}",cargo_toml_file.display(), err));
            }
        }
    }
// }
    // pub load 

    pub fn digest(cargo_toml_file: &Path, manifest : &CargoManifest) -> Result<Manifest> {

        let package = if let Some(package) = &manifest.package {
            package
        } else {
            return Err(error!("package section is missing"));
        };

        let name = package.name.clone();
        let version = Version::from_str(&get_inheritable(&package.version,"version")?)?;

        let url = if let Some(repository) = &package.repository {
            Some(get_inheritable(&repository, "repository")?)
        } else { None };

        // manifest.package.unwrap().version.get()?.try_into()?;
        let mut dependencies: Vec<Dependency> = Vec::new();
        for (name, dep) in manifest.dependencies.iter() {
            match dep {
                CargoDependency::Detailed(detail) => {
                    // println!("{} -> detail: {:?}",name,detail);
                    dependencies.push((name.as_str(),detail).try_into()?);

                },
                CargoDependency::Simple(version) => {
                    // println!("{} -> version: {:?}",name, version);
                    dependencies.push((name.as_str(),version.as_str()).try_into()?);
                },
                CargoDependency::Inherited(_inherited) => {
                    // println!("{} -> inherited: {:?}", name, inherited);
                    return Err(error!("workspace-inherited dependencies are not currently supported"))
                }
            }
        }

        // println!("---\n{:#?}\n---\n\n", manifest);
        Ok(Manifest {
            name,
            cargo_toml_file : cargo_toml_file.to_path_buf(),
            version,
            dependencies,
            url
        })
    }



}


pub fn get_inheritable<T>(v : &Inheritable<T>, name: &str) -> Result<T>
where T : Clone
{
    match v {
        Inheritable::Set(v) => Ok(v.clone()),
        Inheritable::Inherited { workspace } => {
            return Err(error!("inherited values are not supported (`{}`)",name))
        }
    }
} 