use std::env::current_dir;
use serde_derive::Deserialize;
use std::path::Path;
use async_std::fs::*;
use crate::result::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    // pub package: Option<PackageConfig>,
    // pub emanate: EmanateConfig,
    // pub project : ProjectConfig,
    pub repository: Vec<RepositoryConfig>,
    pub build: Option<Vec<BuildConfig>>,
    pub run: Option<RunConfig>,
}

impl Manifest {
    // pub fn application_name(&self) -> String {
    //     match &self.emanate.name {
    //         Some(name) => name.clone(),
    //         None => {
    //             println!("WARNING: manifest is missing [emanate].name section");
    //             self.package.name.clone()
    //         }
    //     }
    // }

    // pub fn application_ident(&self) -> String {
    //     self.package.name.clone()
    // }


    pub async fn load() -> Result<Manifest> {
        let cwd = current_dir().unwrap();
    
        let emanate_toml = read_to_string(cwd.clone().join("Emanate.toml")).await?;
        // println!("toml: {:#?}", toml);
        let manifest: Manifest = match toml::from_str(&emanate_toml) {
            Ok(manifest) => manifest,
            Err(err) => {
                panic!("Error loading Emanate.toml: {}", err);
            }
        };    

        Ok(manifest)
    }
}

// #[derive(Debug, Clone, Deserialize)]
// pub struct PackageConfig {
//     pub name: String,
//     pub version: String,
//     pub authors: Vec<String>,
//     pub description: Option<String>,
//     // port: Option<u64>,
// }



// #[derive(Debug, Clone, Deserialize)]
// pub struct ProjectConfig {
// }



#[derive(Debug, Clone, Deserialize)]
pub struct EmanateConfig {
    pub name: Option<String>,
    pub version: Option<String>,
    pub resources: Option<String>,
    // port: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepositoryConfig {
    pub url: String,
    pub branch: Option<String>,
    pub settings: Option<Vec<String>>,
    // port: Option<u64>,
}

impl RepositoryConfig {
    pub fn name(&self) -> String {
        Path::new(&self.url).file_name().unwrap().to_os_string().into_string().unwrap()
    }

    #[allow(dead_code)]
    pub fn is_external(&self) -> bool {
        if let Some(settings) = self.settings.as_ref() {
            settings.contains(&"external".to_string())
        } else {
            false
        }
    }

    pub fn exists(&self) -> bool {
        Path::new(&self.name()).exists()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BuildConfig {
    pub cmd: String,
    pub folder: String,
}

impl BuildConfig {

}


#[derive(Debug, Clone, Deserialize)]
pub struct RunConfig {
    pub cmd: String,
    pub folder: String,
}

impl RunConfig {

}

