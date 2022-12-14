use std::env::current_dir;
use serde_derive::Deserialize;
use std::fs::*;
use crate::result::Result;
// use crate::repository::Repository;
use crate::build::Build;
use crate::run::Run;
use std::path::*;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub project : ProjectConfig,
    pub publish : PublishConfig,
    // pub repository: Vec<Repository>,
    // pub build: Option<Vec<Build>>,
    // pub run: Option<Run>,
}

impl Config {

    pub fn load(toml_file: &Path) -> Result<Config> {
        // let cwd = current_dir().unwrap();
    
        let text = read_to_string(toml_file)?;
        // println!("toml: {:#?}", toml);
        let manifest: Config = match toml::from_str(&text) {
            Ok(manifest) => manifest,
            Err(err) => {
                panic!("Error loading Emanate.toml: {}", err);
            }
        };    

        Ok(manifest)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectConfig {
    pub root : Option<String>,
    pub repositories: Vec<RepositoryConfig>,
    pub build: Option<Vec<Build>>,
    pub run: Option<Vec<Run>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepositoryConfig {
    pub url: String,
    pub branch: Option<String>,
    pub settings: Option<Vec<String>>,
    pub external: Option<bool>,
    pub publish: Option<bool>,
}


#[derive(Debug, Clone, Deserialize)]
pub struct PublishConfig {
    pub delay : Option<String>
}


