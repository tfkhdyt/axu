use std::io;

use duct::cmd;

use crate::utils::parsing;

pub fn get_explicit_packages() -> io::Result<Vec<String>> {
    let explicit_packages = cmd!("yay", "-Qe")
        .pipe(cmd!("awk", "-F", " ", "{print $1}"))
        .pipe(cmd!("sort"))
        .read()?;
    let splited_packages =
        parsing::vec_str_to_vec_string(explicit_packages.split('\n').collect::<Vec<&str>>());
    Ok(splited_packages)
}
