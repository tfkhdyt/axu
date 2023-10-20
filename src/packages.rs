use std::{thread, time::Duration};

use anyhow::Context;
use indicatif::ProgressBar;

pub fn get_explicit_packages(pb: &ProgressBar) -> anyhow::Result<Vec<String>> {
    pb.set_message("fetch explicit packages");
    let explicit_packages = duct::cmd!("yay", "-Qe")
        .pipe(duct::cmd!("awk", "-F", " ", "{print $1}"))
        .read()
        .context("failed to get explicit packages")?;
    let splited_packages: Vec<String> = explicit_packages
        .split('\n')
        .map(|s| s.to_owned())
        .collect();
    pb.inc(1);
    thread::sleep(Duration::from_millis(100));

    Ok(splited_packages)
}
