use crate::prelude::*;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use toml_edit::{value, Document};

#[derive(Debug, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub suffix: Option<String>,
}

impl Version {
    #[allow(dead_code)]
    pub fn new(major: u32, minor: u32, patch: u32) -> Result<Version> {
        Ok(Version {
            major,
            minor,
            patch,
            suffix: None,
        })
    }

    pub fn change(&mut self, change: &Change) {
        match change {
            Change::Major => {
                self.major += 1;
                self.minor = 0;
                self.patch = 0;
            }
            Change::Minor => {
                self.minor += 1;
                self.patch = 0;
            }
            Change::Patch => self.patch += 1,
            Change::Custom(v) => *self = v.clone(),
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(suffix) = &self.suffix {
            write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, suffix)?;
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        }
        Ok(())
    }
}

impl std::cmp::Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
            .then(other.suffix.cmp(&self.suffix))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
            && self.suffix == other.suffix
    }
}

impl Eq for Version {}

impl FromStr for Version {
    type Err = Error;
    fn from_str(s: &str) -> Result<Version> {
        if s == "*" {
            Err(Error::VersionAsterisk)
        } else {
            let parts = s.split('-').collect::<Vec<_>>();
            let suffix = if parts.len() == 2 {
                Some(parts[1].to_owned())
            } else {
                None
            };
            let v = parts[0].split('.').collect::<Vec<_>>();
            if v.len() != 3 {
                // log_warn!("Warning","detected non-fixed version: `{s}`");
                return Err(Error::NonFixedVersion(s.to_string()));
            }
            Ok(Version {
                major: v[0].parse()?,
                minor: v[1].parse()?,
                patch: v[2].parse()?,
                suffix,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub enum Change {
    Major,
    Minor,
    Patch,
    Custom(Version),
}

impl FromStr for Change {
    type Err = Error;
    fn from_str(s: &str) -> Result<Change> {
        match s {
            "major" => Ok(Change::Major),
            "minor" => Ok(Change::Minor),
            "patch" => Ok(Change::Patch),
            _ => Ok(Change::Custom(s.parse()?)),
        }
    }
}

pub struct Versioner {
    ctx: Context,
}

impl Versioner {
    pub fn new(ctx: Context) -> Versioner {
        Versioner { ctx }
    }

    pub fn change(&self, change: Change) -> Result<()> {
        match &self.ctx {
            Context::Workspace(ctx) => {
                let mut version = ctx.manifest.version()?;
                version.change(&change);
                let mut doc = ctx.manifest.toml.parse::<Document>().unwrap_or_else(|err| {
                    panic!("toml edit - unable to parse workspace manifest: {err}")
                });

                let v = version.to_string();
                doc["workspace"]["package"]["version"] = value(&v);

                for dep in ctx.manifest.workspace.dependencies.keys() {
                    doc["workspace"]["dependencies"][dep]["version"] = value(&v);
                }

                let doc_str = doc.to_string();
                fs::write(&ctx.manifest.file, doc_str)?;
            }
            _ => {
                panic!("not currently supported in a single crate context");
            }
        }

        Ok(())
    }
}
