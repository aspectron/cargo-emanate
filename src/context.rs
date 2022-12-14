use crate::prelude::*;
use std::env;
use std::path::*;

#[derive(Debug)]
pub struct Context {
    pub manifest : Config,
    pub root_folder : PathBuf,
    pub toml_file_location : PathBuf,


}

impl Context {
    pub fn try_new(location: Option<String>) -> Result<Self> {

        let location = location.map(|l|Path::new(&l).to_path_buf()).unwrap_or(env::current_dir()?);

        let extension = location.extension();
        let (toml_file_location,toml_file) = match extension {
            Some(extension) if extension == "toml" => {
                (location.parent().unwrap().to_path_buf(), location)
            },
            _ => {
                (location.clone(), location.join("emanate.toml"))
            }
        };

        let manifest = Config::load(&toml_file)?;

        let root_folder = if let Some(root) = &manifest.project.root {
            let root = if root.starts_with("~/") {
                let home_dir = home::home_dir().unwrap();
                home_dir.join(root.split_at(2).1)
            } else {
                PathBuf::from(root)
            };
            // if root.starts_with("~/") || 
            if root.starts_with("/") {
                Path::new(&root).to_path_buf()
            } else {
                toml_file_location.join(root)
            }
        } else {
            toml_file_location.clone()
        };

        if !root_folder.is_dir() {
            return Err(error!("Unable to locate root folder `{}`", root_folder.display()));
        }

        // Context::load_projects(&manifest)?;



        let ctx = Context {
            manifest,
            root_folder,
            toml_file_location,
        };

        Ok(ctx)
    }

    // fn load_cargo_manifests(manifest: &Manifest) -> Result<> {

    // }
}