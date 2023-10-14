use colored::*;

use crate::{updates::UpdateType, versions};

#[derive(Default)]
pub struct UpdateTypeMap {
    pub major: Vec<String>,
    pub minor: Vec<String>,
    pub patch: Vec<String>,
    pub build: Vec<String>,
}

pub fn format_result(
    common_lines: &Vec<String>,
    update_types: &Option<Vec<UpdateType>>,
) -> UpdateTypeMap {
    let mut update_type_map = UpdateTypeMap::default();

    for line in common_lines {
        let ln = line.split_whitespace().collect::<Vec<&str>>();

        let package_name = ln[0];
        let old_version = ln[1];
        let new_version = ln[3];

        let update_type = versions::compare_version(old_version, new_version);
        if let Some(update_type_arg) = update_types {
            if !update_type_arg.contains(&update_type) {
                continue;
            }
        }
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

pub fn print_result(update_type_map: UpdateTypeMap, show_number_only: bool) {
    let mut number_of_updates = 0;

    if !update_type_map.major.is_empty() {
        number_of_updates += update_type_map.major.len();
        if !show_number_only {
            println!(
                "{} ({})",
                "MAJOR".bold().underline(),
                update_type_map.major.len().to_string().bright_green(),
            );
            update_type_map
                .major
                .iter()
                .for_each(|it| println!("{}", it));
            println!();
        }
    }
    if !update_type_map.minor.is_empty() {
        number_of_updates += update_type_map.minor.len();
        if !show_number_only {
            println!(
                "{} ({})",
                "MINOR".bold().underline(),
                update_type_map.minor.len().to_string().bright_green(),
            );
            update_type_map
                .minor
                .iter()
                .for_each(|it| println!("{}", it));
            println!();
        }
    }
    if !update_type_map.patch.is_empty() {
        number_of_updates += update_type_map.patch.len();
        if !show_number_only {
            println!(
                "{} ({})",
                "PATCH".bold().underline(),
                update_type_map.patch.len().to_string().bright_green(),
            );
            update_type_map
                .patch
                .iter()
                .for_each(|it| println!("{}", it));
            println!();
        }
    }
    if !update_type_map.build.is_empty() {
        number_of_updates += update_type_map.build.len();
        if !show_number_only {
            println!(
                "{} ({})",
                "BUILD".bold().underline(),
                update_type_map.build.len().to_string().bright_green(),
            );
            update_type_map
                .build
                .iter()
                .for_each(|it| println!("{}", it));
            println!();
        }
    }

    if show_number_only {
        println!("{}", number_of_updates)
    } else {
        println!("{} {}", "Total updates:".bold(), number_of_updates);
    }
}
