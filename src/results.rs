use colored::*;

use crate::{updates::UpdateType, versions};

#[derive(Default)]
pub struct UpdateTypeMap {
    pub major: Vec<String>,
    pub minor: Vec<String>,
    pub patch: Vec<String>,
    pub build: Vec<String>,
}

pub fn format_result(common_lines: &Vec<String>, update_types: &Vec<UpdateType>) -> UpdateTypeMap {
    let mut update_type_map = UpdateTypeMap::default();

    for line in common_lines {
        let ln = line.split_whitespace().collect::<Vec<&str>>();

        let package_name = ln[0];
        let old_version = ln[1];
        let new_version = ln[3];

        let update_type = versions::compare_version(old_version, new_version);
        if !update_types.is_empty() && !update_types.contains(&update_type) {
            continue;
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
    let updates_types = [
        &update_type_map.major,
        &update_type_map.minor,
        &update_type_map.patch,
        &update_type_map.build,
    ];

    for (idx, update_type) in updates_types.iter().enumerate() {
        if !update_type.is_empty() {
            number_of_updates += update_type.len();
            if !show_number_only {
                println!(
                    "{} ({})",
                    match idx {
                        0 => "MAJOR",
                        1 => "MINOR",
                        2 => "PATCH",
                        _ => "BUILD",
                    }
                    .bold()
                    .underline(),
                    update_type.len().to_string().bright_green(),
                );
                update_type.iter().for_each(|it| println!("{}", it));
                println!();
            }
        }
    }

    if show_number_only {
        println!("{}", number_of_updates)
    } else {
        println!("{} {}", "Total updates:".bold(), number_of_updates);
    }
}
