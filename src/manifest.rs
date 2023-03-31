use crate::prelude::*;
use serde::{Deserialize, Serialize};
use toml::*;

#[derive(Debug, Clone, Deserialize)]
pub struct Crate {
    #[serde(skip)]
    pub file: PathBuf,
    #[serde(skip)]
    pub folder: PathBuf,
    #[serde(skip)]
    pub toml: String,
    #[serde(skip)]
    pub toml_root: Option<Value>,
    pub package: Package,
    // #[serde(skip)]
    pub dependencies: Dependencies,
}

impl Crate {
    pub async fn load(file: &PathBuf) -> Result<Crate> {
        let toml = async_std::fs::read_to_string(&file).await?;
        let mut crt: Crate = toml::from_str(&toml)?;
        let table: Value = toml::from_str(&toml)?;

        let targets = table.get("target");
        if let Some(targets) = targets {
            if let Some(targets) = targets.as_table() {
                for (_k, v) in targets.iter() {
                    if let Some(cfgs) = v.as_table() {
                        for (k2, v2) in cfgs.iter() {
                            if k2 == "dependencies" {
                                if let Some(deps) = v2.as_table() {
                                    for (k3, v3) in deps.iter() {
                                        crt.dependencies.insert(k3.clone(), Dependency(v3.clone()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // println!("{:#?}",table);
        // panic!();
        let folder = file.parent().unwrap_or_else(|| {
            panic!(
                "unable to determin parent folder for location: {}",
                file.display()
            )
        });

        crt.toml_root = Some(table);
        crt.file = file.to_owned();
        crt.folder = folder.to_owned();
        crt.toml = toml;
        Ok(crt)
    }

    pub fn name(&self) -> &str {
        &self.package.name
    }

    pub fn toml_root(&self) -> &Value {
        self.toml_root.as_ref().unwrap()
    }

    pub fn metadata(&self) -> Result<Option<Metadata>> {
        if let Some(Some(package)) = self.toml_root().get("package").map(Value::as_table) {
            if let Some(Some(metadata)) = package.get("metadata").map(Value::as_table) {
                if let Some(emanate) = metadata.get("emanate") {
                    let metadata = Metadata::deserialize(emanate.clone())?;
                    return Ok(Some(metadata));
                }
            }
        }
        Ok(None)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub wasm: Option<WasmMetadata>,
    pub build: Option<BuildMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: Value,
    pub publish: Option<bool>,
    pub metadata: Option<Value>,
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
    pub fn git(&self) -> Option<()> {
        match &self.0 {
            Value::Table(table) => {
                let git = table.get("git");
                git.map(|_| ())
            }
            _ => None,
        }
    }
    pub fn version(&self) -> Result<Version> {
        match &self.0 {
            Value::String(s) => Ok(s.parse().map_err(|err| error!("{err}"))?),
            Value::Table(table) => {
                let version = table.get("version");
                if let Some(version) = version {
                    Ok(version
                        .as_str()
                        .ok_or_else(|| error!("version is not a string: {version:?}"))?
                        .parse()
                        .map_err(|err| error!("{err}"))?)
                } else if table.get("workspace").is_some() {
                    Err(Error::WorkspaceCrate)
                } else if table.get("path").is_some() {
                    Err(Error::RelativeCrate)
                } else {
                    Err(error!("dependency is missing version property"))
                }
            }
            _ => Err(error!("dependency is not a string or a table")),
        }
    }
}

impl Manifest {
    pub async fn load(file: &PathBuf) -> Result<Manifest> {
        let toml = fs::read_to_string(file)?;
        let mut manifest: Manifest = toml::from_str(&toml)?;
        manifest.file = file.to_owned();
        manifest.toml = toml;
        Ok(manifest)
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WasmTargetPlatform {
    Web,
    NodeJs,
}

impl ToString for WasmTargetPlatform {
    fn to_string(&self) -> String {
        match self {
            WasmTargetPlatform::Web => "web",
            WasmTargetPlatform::NodeJs => "nodejs",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmTarget {
    pub target: WasmTargetPlatform,
    #[serde(rename = "out-dir")]
    pub out_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmMetadata {
    pub targets: Vec<WasmTarget>,
    pub folder: Option<String>,
    pub docs: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildMetadata {
    pub folder: Option<String>,
    // pub targets: Vec<WasmTarget>,
    // pub docs: Option<String>,
}
