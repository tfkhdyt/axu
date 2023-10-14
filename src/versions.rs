use colored::*;

use crate::updates::{self, UpdateType};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    build: String,
}

fn get_version_parts(version: &str) -> Version {
    let parts: Vec<&str> = version.split(|c| c == '.' || c == '-').collect();
    let major = parts[0].parse().unwrap_or(0);
    let minor = parts.get(1).map_or(0, |v| v.parse().unwrap_or(0));
    let patch = parts.get(2).map_or(0, |v| v.parse().unwrap_or(0));
    let build = parts.get(3).map_or("", |v| *v).to_string();

    Version {
        major,
        minor,
        patch,
        build,
    }
}

pub fn compare_version(old_version: &str, new_version: &str) -> UpdateType {
    let old_parts = get_version_parts(old_version);
    let new_parts = get_version_parts(new_version);

    if old_parts.major != new_parts.major {
        updates::UpdateType::Major
    } else if old_parts.minor != new_parts.minor {
        updates::UpdateType::Minor
    } else if old_parts.patch != new_parts.patch {
        updates::UpdateType::Patch
    } else {
        updates::UpdateType::Build
    }
}

pub fn format_version_color(version: &str, update_type: &UpdateType) -> String {
    let ver_parts = version.split('.').collect::<Vec<&str>>();

    match update_type {
        UpdateType::Major => ver_parts.join(".").red().to_string(),
        UpdateType::Minor => format!(
            "{}.{}",
            ver_parts[0],
            &ver_parts[1..].join(".").bold().yellow()
        ),
        UpdateType::Patch => format!(
            "{}.{}.{}",
            ver_parts[0],
            ver_parts[1],
            ver_parts[2..].join(".").bold().green()
        ),
        UpdateType::Build => ver_parts.join("."),
    }
}
