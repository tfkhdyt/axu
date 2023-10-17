use anyhow::Context;

pub fn get_explicit_packages() -> anyhow::Result<Vec<String>> {
    let explicit_packages = duct::cmd!("yay", "-Qe")
        .pipe(duct::cmd!("awk", "-F", " ", "{print $1}"))
        .read()
        .context("failed to get explicit packages")?;
    let splited_packages: Vec<String> = explicit_packages
        .split('\n')
        .map(|s| s.to_owned())
        .collect();
    Ok(splited_packages)
}
