use colored::*;

use crate::updates::{self, UpdateType};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    build: String,
    parts: Vec<String>,
    raw: String,
}

impl Version {
    pub fn new(version: &str) -> Self {
        let parts: Vec<String> = version
            .split(|c| c == '.' || c == '-')
            .map(|p| p.to_owned())
            .collect();

        let major = parts[0].parse().unwrap_or(0);
        let minor = parts.get(1).map_or(0, |v| v.parse().unwrap_or(0));
        let patch = parts.get(2).map_or(0, |v| v.parse().unwrap_or(0));
        let build = parts.get(3).map_or("", |v| v).to_owned();

        Version {
            major,
            minor,
            patch,
            build,
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
            UpdateType::Major => self.parts.join(".").red().to_string(),
            UpdateType::Minor => format!(
                "{}.{}",
                self.parts[0],
                &self.parts[1..].join(".").bold().yellow()
            ),
            UpdateType::Patch => format!(
                "{}.{}.{}",
                self.parts[0],
                self.parts[1],
                self.parts[2..].join(".").bold().green()
            ),
            UpdateType::Build => format!(
                "{}-{}",
                self.parts[0..self.parts.len() - 1].join("."),
                self.parts[self.parts.len() - 1].bold().blue()
            ),
            UpdateType::Git => self.raw.to_string(),
        }
    }
}
