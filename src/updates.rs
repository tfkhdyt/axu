use anyhow::Context;
use clap::ValueEnum;
use indicatif::ProgressBar;

#[derive(ValueEnum, Clone, PartialEq, PartialOrd)]
pub enum UpdateType {
    Major,
    Minor,
    Patch,
    Build,
    Git,
}

pub fn get_all_updates(pb: &ProgressBar) -> anyhow::Result<Vec<String>> {
    let (arch_updates, aur_updates) = rayon::join(
        || -> anyhow::Result<String> {
            pb.set_message("fetching arch linux updates");
            let result = duct::cmd!("checkupdates")
                .read()
                .context("failed to get Arch Linux updates")?;
            pb.inc(1);
            Ok(result)
        },
        || -> anyhow::Result<String> {
            pb.set_message("fetching aur updates");
            let result = duct::cmd!("yay", "-Qua")
                .unchecked()
                .read()
                .context("failed to get AUR updates")?;
            pb.inc(1);
            Ok(result)
        },
    );

    let all_updates = format!("{}\n{}", arch_updates?, aur_updates?);
    let mut splited_vec: Vec<String> = all_updates.split('\n').map(|s| s.to_owned()).collect();
    splited_vec.sort();

    Ok(splited_vec)
}
