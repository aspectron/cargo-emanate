use crate::prelude::*;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use toml_edit::{value, Document};

#[derive(Debug, Clone, Copy)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    #[allow(dead_code)]
    pub fn new(major: u32, minor: u32, patch: u32) -> Result<Version> {
        Ok(Version {
            major,
            minor,
            patch,
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
            Change::Custom(v) => *self = *v,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        Ok(())
    }
}

impl std::cmp::Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl Eq for Version {}

impl FromStr for Version {
    type Err = Error;
    fn from_str(s: &str) -> Result<Version> {
        let v = s.split('.').collect::<Vec<_>>();
        if v.len() != 3 {
            return Err(Error::InvalidVersion(s.to_string()));
        }
        Ok(Version {
            major: v[0].parse()?,
            minor: v[1].parse()?,
            patch: v[2].parse()?,
        })
    }
}

#[derive(Debug)]
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
    ctx: Arc<Context>,
}

impl Versioner {
    pub fn new(ctx: &Arc<Context>) -> Versioner {
        Versioner { ctx: ctx.clone() }
    }

    pub fn change(&self, change: Change) -> Result<()> {
        let mut version = self.ctx.manifest.version()?;
        version.change(&change);
        let mut doc = self
            .ctx
            .manifest
            .toml
            .parse::<Document>()
            .unwrap_or_else(|err| panic!("toml edit - unable to parse workspace manifest: {err}"));

        let v = version.to_string();
        doc["workspace"]["package"]["version"] = value(&v);

        for dep in self.ctx.manifest.workspace.dependencies.keys() {
            doc["workspace"]["dependencies"][dep]["version"] = value(&v);
        }

        let doc_str = doc.to_string();
        fs::write(&self.ctx.manifest.file, doc_str)?;

        Ok(())
    }
}
