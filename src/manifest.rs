use crate::prelude::*;
use toml::*;

#[derive(Debug, Deserialize)]
pub struct Crate {
    #[serde(skip)]
    pub file: PathBuf,
    #[serde(skip)]
    pub toml: String,
    pub package: Package,
    pub dependencies: Dependencies,
}

impl Crate {
    pub async fn load(file: &PathBuf) -> Result<Crate> {
        let toml = async_std::fs::read_to_string(&file).await?;
        let mut crt: Crate = toml::from_str(&toml)?;
        crt.file = file.to_owned();
        crt.toml = toml;
        Ok(crt)
    }
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: Value,
    pub publish: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    #[serde(skip)]
    pub file: PathBuf,
    #[serde(skip)]
    pub toml: String,
    pub workspace: Workspace,
}

impl Manifest {
    pub fn version(&self) -> Result<Version> {
        self.workspace.package.version.parse()
    }
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub members: Vec<String>,
    pub package: WorkspacePackage,
    pub dependencies: Dependencies,
}

pub type Dependencies = HashMap<String, Dependency>;

#[derive(Debug, Deserialize)]
pub struct WorkspacePackage {
    pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dependency(Value);

impl Dependency {
    pub fn version(&self) -> Result<Version> {
        match &self.0 {
            Value::String(s) => Ok(s.parse().map_err(|err| error!("{err}: `{s}`"))?),
            Value::Table(table) => {
                let version = table.get("version");
                if let Some(version) = version {
                    Ok(version
                        .as_str()
                        .ok_or_else(|| error!("version is not a string: {version:?}"))?
                        .parse()
                        .map_err(|err| error!("{err}: `{version}`"))?)
                } else {
                    Err(error!("dependency is missing version property"))
                }
            }
            _ => Err(error!("dependency is not a string or a table")),
        }
    }
}

impl Manifest {
    pub async fn locate(location: Option<String>) -> Result<PathBuf> {
        let cwd = current_dir();

        let location = if let Some(location) = location {
            if let Some(stripped) = location.strip_prefix("~/") {
                home::home_dir()
                    .expect("unable to get home directory")
                    .join(stripped)
            } else {
                let location = Path::new(&location).to_path_buf();
                if location.is_absolute() {
                    location
                } else {
                    cwd.join(&location)
                }
            }
        } else {
            cwd
        };

        let locations = [&location, &location.join("Cargo.toml")];

        for location in locations.iter() {
            if let Ok(location) = location.canonicalize() {
                if location.is_file() {
                    return Ok(location);
                }
            }
        }

        Err(error!("Unable to locate 'Cargo.toml' manifest"))
    }

    pub async fn load(file: &PathBuf) -> Result<Manifest> {
        let toml = fs::read_to_string(file)?;
        let mut manifest: Manifest = toml::from_str(&toml)?;
        manifest.file = file.to_owned();
        manifest.toml = toml;
        Ok(manifest)
    }
}
