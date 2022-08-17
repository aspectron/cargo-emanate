use std::env::current_dir;
use serde_derive::Deserialize;
// use std::path::PathBuf;
use async_std::{fs::*, path::Path};
// use toml::from_str;
use crate::result::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    // pub package: Option<PackageConfig>,
    // pub emanate: EmanateConfig,
    pub repository: Vec<RepositoryConfig>,
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
        // println!("cargo_toml: {:#?}", cargo_toml);
        let manifest: Manifest = match toml::from_str(&emanate_toml) {
            Ok(manifest) => manifest,
            Err(err) => {
                panic!("Error loading Cargo.toml: {}", err);
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
    // port: Option<u64>,
}

impl RepositoryConfig {
    pub fn name(&self) -> String {
        Path::new(&self.url).file_name().unwrap().to_os_string().into_string().unwrap()
    }
}