use crate::prelude::*;
use cargo_toml::{Manifest,Dependency as DependencyImpl};

// pub struct ManifestData {
//     pub 
// }

#[derive(Debug, Clone)]
pub struct Repository {
    pub ctx : Arc<Context>,
    pub url: String,
    // pub config : RepositoryConfig
    pub branch: Option<String>,
    pub name: String,
    pub folder : PathBuf,
    pub external : bool,
    pub publish : bool,
    pub config : RepositoryConfig,
    // pub settings: Option<Vec<String>>,
    // port: Option<u64>,
}

impl TryFrom<(&Arc<Context>,&RepositoryConfig)> for Repository {
    type Error = Error;
    fn try_from((ctx,config): (&Arc<Context>,&RepositoryConfig)) -> Result<Self> {

        let url = config.url.clone();
        let branch = config.branch.clone();
        let external = config.external.unwrap_or(false);
        let publish = config.publish.unwrap_or(!external);
        let name = Path::new(&url).file_name().unwrap().to_os_string().into_string().unwrap();
        let folder = ctx.root_folder.join(&name);

        let repository = Repository {
            url,
            branch,
            external,
            publish,
            name,
            folder,
            config : config.clone(),
            ctx : ctx.clone(),
        };

        Ok(repository)
    }
}

impl Repository {
    // pub fn name(&self) -> String {
    //     Path::new(&self.url).file_name().unwrap().to_os_string().into_string().unwrap()
    // }

    pub fn is_external(&self) -> bool {
        self.external
    }
    pub fn is_published(&self) -> bool {
        self.publish
    }
    // #[allow(dead_code)]
    // pub fn is_external(&self) -> bool {
    //     if let Some(settings) = self.config.as_ref() {
    //         settings.contains(&"external".to_string())
    //     } else {
    //         false
    //     }
    // }

    pub fn exists(&self) -> bool {
        self.folder.is_dir()
    }

    // pub async fn clone(&self) -> Result<&Self> {
    //     if self.exists() {
    //         println!("{} repository {} exists. skipping...",style("WARNING:").magenta(),style(self.name()).cyan()); 
    //     } else {
    //         println!("{} ...",style(self.name()).cyan()); 
    //         match &self.branch {
    //             Some(branch) => {
    //                 cmd!("git","clone","-b",branch, &self.url).run()?;
    //             },
    //             None => {
    //                 cmd!("git","clone", &self.url).run()?;
    //             }
    //         }
    //     }
    
    //     Ok(self)
    // }

    // pub async fn pull(&self) -> Result<&Self> {
    //     if !self.exists() {
    //         self.clone().await?;
    //     } else {
    //         let folder = self.name();
    //         println!("pulling {}...", folder);
    //         cmd!("git","pull").dir(folder).run()?;
    //     }

    //     Ok(self)
    // }

    pub async fn sync(&self) -> Result<&Self> {
        if !self.exists() {
            println!("cloning {} ...",style(&self.name).cyan()); 
            match &self.branch {
                Some(branch) => {
                    cmd!("git","clone","-b",branch, &self.url).run()?;
                },
                None => {
                    cmd!("git","clone", &self.url).run()?;
                }
            }
        } else {
            // let folder = self.name();
            println!("pulling {} ...", self.name);
            cmd!("git","pull").dir(&self.folder).run()?;
        }
    
        Ok(self)
    }

    // pub fn load_cargo_manifest(&self) -> Result<()> {

    //     if self.external {
    //         return Ok(());
    //     }

    //     let cargo_toml_path = self.folder.join("Cargo.toml");
    //     // let manifest = Manifest::load(&manifest_path)?;

    //     let manifest = match Manifest::from_path(&cargo_toml_path) {
    //         Ok(manifest) => manifest,
    //         Err(err) => {
    //             return Err(error!("Unable to process `{}`: {}",cargo_toml_path.display(),err));
    //         }
    //     };

    //     println!("\n\nCargo.toml at `{}`",cargo_toml_path.display());

    //     for (name, dep) in manifest.dependencies.iter() {
    //         match dep {
    //             DependencyImpl::Detailed(detail) => {
    //                 println!("{} -> detail: {:?}",name,detail);
    //             },
    //             DependencyImpl::Simple(version) => {
    //                 println!("{} -> version: {:?}",name, version);
    //             },
    //             DependencyImpl::Inherited(inherited) => {
    //                 println!("{} -> inherited: {:?}", name, inherited);
    //             }
    //         }
    //     }

    //     // println!("---\n{:#?}\n---\n\n", manifest);
    //     Ok(())
    // }

}
