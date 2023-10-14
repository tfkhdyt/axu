use colored::*;

use crate::{updates::UpdateType, versions};

#[derive(Default)]
pub struct UpdateTypeMap {
    pub major: Vec<String>,
    pub minor: Vec<String>,
    pub patch: Vec<String>,
    pub build: Vec<String>,
}

pub fn format_result(common_lines: &Vec<String>) -> UpdateTypeMap {
    let mut update_type_map = UpdateTypeMap::default();

    for line in common_lines {
        let ln = line.split_whitespace().collect::<Vec<&str>>();

        let package_name = ln[0];
        let old_version = ln[1];
        let new_version = ln[3];

        let update_type = versions::compare_version(old_version, new_version);
        let vec = match update_type {
            UpdateType::Major => &mut update_type_map.major,
            UpdateType::Minor => &mut update_type_map.minor,
            UpdateType::Patch => &mut update_type_map.patch,
            UpdateType::Build => &mut update_type_map.build,
        };
        vec.push(format!(
            "{}: {}  {}",
            package_name.bold(),
            versions::format_version_color(old_version, &update_type),
            versions::format_version_color(new_version, &update_type)
        ));
    }

    update_type_map
}

pub fn print_result(update_type_map: UpdateTypeMap) {
    if !update_type_map.major.is_empty() {
        println!(
            "{} ({})",
            "MAJOR".bold().underline(),
            update_type_map.major.len().to_string().bright_green(),
        );
        update_type_map
            .major
            .into_iter()
            .for_each(|it| println!("{}", it));
        println!()
    }
    if !update_type_map.minor.is_empty() {
        println!(
            "{} ({})",
            "MINOR".bold().underline(),
            update_type_map.minor.len().to_string().bright_green(),
        );
        update_type_map
            .minor
            .into_iter()
            .for_each(|it| println!("{}", it));
        println!()
    }
    if !update_type_map.patch.is_empty() {
        println!(
            "{} ({})",
            "PATCH".bold().underline(),
            update_type_map.patch.len().to_string().bright_green(),
        );
        update_type_map
            .patch
            .into_iter()
            .for_each(|it| println!("{}", it));
        println!()
    }
    if !update_type_map.build.is_empty() {
        println!(
            "{} ({})",
            "BUILD".bold().underline(),
            update_type_map.build.len().to_string().bright_green(),
        );
        update_type_map
            .build
            .into_iter()
            .for_each(|it| println!("{}", it));
        println!()
    }
}
