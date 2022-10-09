use serde_derive::Deserialize;
use crate::result::Result;
use duct::cmd;

#[derive(Debug, Clone, Deserialize)]
pub struct Build {
    pub cmd: String,
    pub folder: String,
}

impl Build {

    pub async fn execute(&self) -> Result<&Self> {
        let cwd = std::env::current_dir()?;

        let argv : Vec<String> = self.cmd.split(" ").map(|s|s.to_string()).collect();
        let program = argv.first().expect("missing program in build config");
        let args = argv[1..].to_vec();
        cmd(program,args).dir(cwd.join(&self.folder)).run()?;

        Ok(self)
    }

}
