mod packages;
mod results;
mod updates;
mod utils;
mod versions;

use colored::*;

use crate::utils::{fmt, lines};

fn main() {
    let (all_updates, explicit_packages) =
        rayon::join(updates::get_all_updates, packages::get_explicit_packages);

    let all_updates = match all_updates {
        Ok(v) => v,
        Err(err) => fmt::fatalln("failed to get all updates", Some(&err)),
    };
    let explicit_packages = match explicit_packages {
        Ok(v) => v,
        Err(err) => fmt::fatalln("Failed to get explicit packages", Some(&err)),
    };

    let common_lines = lines::get_common_lines(all_updates, explicit_packages);
    let result = results::format_result(&common_lines);

    results::print_result(result);

    println!("{} {}", "Total updates:".bold(), common_lines.len());
}
