use std::collections::HashMap;

use serde_derive::Deserialize;
// use serde::Deserialize;
// use std::env::current_dir;
// use std::fs::*;
// use crate::result::Result;
// use crate::repository::Repository;
// use crate::build::Build;
// use crate::run::Run;
use crate::prelude::*;


// #[derive(Debug, Clone, Deserialize)]
// pub struct Manifest {
//     pub version : String,
//     pub name : String,
//     pub repository : String,
//     pub dependencies : Option<HashMap<String,Dependency>>,
//     // pub description : String,
//     // pub project : ProjectConfig,
//     // pub publish : PublishConfig,
//     // pub repository: Vec<Repository>,
//     // pub build: Option<Vec<Build>>,
//     // pub run: Option<Run>,
// }

// impl Manifest {

//     pub fn load(cargo_toml: &Path) -> Result<Manifest> {
    
//         let text = fs::read_to_string(cargo_toml)?;
//         // println!("toml: {:#?}", toml);
//         let manifest: Manifest = match toml::from_str(&text) {
//             Ok(manifest) => manifest,
//             Err(err) => {
//                 panic!("Error loading `{}`: {}", cargo_toml.display(), err);
//             }
//         };    

//         Ok(manifest)
//     }
// }

// #[derive(Debug, Clone, Deserialize)]
// pub enum Dependency {
//     Version(String),
//     Config(DependencyConfig)
// }



// #[derive(Debug, Clone, Deserialize)]
// pub struct DependencyConfig {
//     pub version : String,
//     pub features : Option<Vec<String>>,
//     pub path : Option<String>,
// }