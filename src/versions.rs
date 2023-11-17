use colored::*;

use crate::updates::{self, UpdateType};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: Option<String>,
    minor: Option<String>,
    patch: Option<String>,
    build: String,
    parts: Vec<String>,
    pub raw: String,
}

impl Version {
    pub fn new(version: &str) -> Self {
        let mut parts: Vec<String> = version
            .split(|c| c == '.' || c == '-')
            .map(|p| p.to_owned())
            .collect();
        let pkgrel = parts.pop().unwrap_or("1".to_owned());

        Version {
            major: parts.get(0).cloned(),
            minor: parts.get(1).cloned(),
            patch: if parts.len() > 2 && !parts[2..].is_empty() {
                Some(parts[2..].join("."))
            } else {
                None
            },
            build: pkgrel,
            parts,
            raw: version.to_owned(),
        }
    }

    pub fn get_update_type(&self, new_version: &Self) -> UpdateType {
        if self.major != new_version.major {
            updates::UpdateType::Major
        } else if self.minor != new_version.minor {
            updates::UpdateType::Minor
        } else if self.patch != new_version.patch {
            updates::UpdateType::Patch
        } else {
            updates::UpdateType::Build
        }
    }

    pub fn format_color(&self, update_type: &UpdateType) -> String {
        match update_type {
            // UpdateType::Major => self.parts.join(".").red().to_string(),
            UpdateType::Major => format!("{}-{}", self.parts.join("."), self.build)
                .red()
                .to_string(),
            UpdateType::Minor => format!(
                "{}.{}{}{}",
                self.parts[0],
                self.parts[1..].join(".").bold().yellow(),
                "-".bold().yellow(),
                self.build.bold().yellow()
            ),
            UpdateType::Patch => format!(
                "{}.{}.{}{}{}",
                self.parts[0],
                self.parts[1],
                self.parts[2..].join(".").bold().green(),
                "-".bold().green(),
                self.build.bold().green()
            ),
            UpdateType::Build => {
                format!("{}-{}", self.parts[0..].join("."), self.build.bold().blue())
            }
            UpdateType::Git => self.raw.to_owned(),
        }
    }
}
