use std::{thread, time::Duration};

use colored::*;
use indicatif::ProgressBar;

use crate::{updates::UpdateType, versions::Version};

#[derive(Default)]
pub struct UpdateTypeMap {
    pub major: Vec<String>,
    pub minor: Vec<String>,
    pub patch: Vec<String>,
    pub build: Vec<String>,
    pub git: Vec<String>,
}

impl UpdateTypeMap {
    pub fn new(common_lines: &[String], update_types: &[UpdateType], pb: &ProgressBar) -> Self {
        pb.set_message("determining the type of update");
        let mut update_type_map = UpdateTypeMap::default();

        for line in common_lines {
            let ln = line.split_whitespace().collect::<Vec<&str>>();

            let package_name = ln[0];
            let old_version = Version::new(ln[1]);
            let new_version = Version::new(ln[3]);

            let update_type = match package_name.ends_with("-git") {
                true => UpdateType::Git,
                false => old_version.get_update_type(&new_version),
            };

            if !update_types.is_empty() && !update_types.contains(&update_type) {
                continue;
            }

            let vec = match update_type {
                UpdateType::Major => &mut update_type_map.major,
                UpdateType::Minor => &mut update_type_map.minor,
                UpdateType::Patch => &mut update_type_map.patch,
                UpdateType::Build => &mut update_type_map.build,
                UpdateType::Git => &mut update_type_map.git,
            };

            vec.push(format!(
                "{}: {}  {}",
                package_name.bold(),
                old_version.format_color(&update_type),
                new_version.format_color(&update_type),
            ));
        }
        pb.inc(1);
        thread::sleep(Duration::from_millis(100));

        update_type_map
    }

    pub fn print(&self, print_number_only: bool) {
        let mut number_of_updates = 0;

        let updates_types = [
            &self.major,
            &self.minor,
            &self.patch,
            &self.build,
            &self.git,
        ];

        for (idx, update_type) in updates_types.iter().enumerate() {
            if !update_type.is_empty() {
                number_of_updates += update_type.len();
                if !print_number_only {
                    println!(
                        "{} ({})",
                        match idx {
                            0 => "MAJOR",
                            1 => "MINOR",
                            2 => "PATCH",
                            3 => "BUILD",
                            _ => "GIT",
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

        if print_number_only {
            println!("{}", number_of_updates)
        } else {
            println!("{} {}", "Total updates:".bold(), number_of_updates);
        }
    }
}
