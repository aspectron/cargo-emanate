use serde_derive::Deserialize;
use std::path::Path;
use crate::result::Result;
use duct::cmd;
use console::style;

#[derive(Debug, Clone, Deserialize)]
pub struct Repository {
    pub url: String,
    pub branch: Option<String>,
    pub settings: Option<Vec<String>>,
    // port: Option<u64>,
}

impl Repository {
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
            println!("cloning {} ...",style(self.name()).cyan()); 
            match &self.branch {
                Some(branch) => {
                    cmd!("git","clone","-b",branch, &self.url).run()?;
                },
                None => {
                    cmd!("git","clone", &self.url).run()?;
                }
            }
        } else {
            let folder = self.name();
            println!("pulling {} ...", folder);
            cmd!("git","pull").dir(folder).run()?;
        }
    
        Ok(self)
    }


}
