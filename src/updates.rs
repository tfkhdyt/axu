use anyhow::Context;
use clap::ValueEnum;

#[derive(ValueEnum, Clone, PartialEq, PartialOrd)]
pub enum UpdateType {
    Major,
    Minor,
    Patch,
    Build,
    Git,
}

pub fn get_all_updates() -> anyhow::Result<Vec<String>> {
    let (arch_updates, aur_updates) = rayon::join(
        || -> anyhow::Result<_> {
            duct::cmd!("checkupdates")
                .read()
                .context("failed to get Arch Linux updates")
        },
        || -> anyhow::Result<_> {
            duct::cmd!("yay", "-Qua")
                .unchecked()
                .read()
                .context("failed to get AUR updates")
        },
    );

    let all_updates = format!("{}\n{}", arch_updates?, aur_updates?);

    let mut splited_vec: Vec<String> = all_updates.split('\n').map(|s| s.to_owned()).collect();

    splited_vec.sort();

    Ok(splited_vec)
}
