mod cli;
mod packages;
mod results;
mod updates;
mod utils;
mod versions;

use std::process;

use clap::{CommandFactory, Parser};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    cli::{Cli, Commands},
    results::UpdateTypeMap,
    utils::{completions, lines},
};

fn main() {
    let cli = Cli::parse();

    if let Some(Commands::Completion { shell }) = &cli.command {
        let mut cmd = Cli::command();
        completions::print_completions(*shell, &mut cmd);
        process::exit(0);
    }

    let pb = ProgressBar::new(5);
    pb.set_style(
        ProgressStyle::with_template("[{wide_bar:.yellow/white}] {percent:<1}% ")
            .unwrap()
            .progress_chars("-C∘"),
    );

    let (all_updates, explicit_packages) = rayon::join(
        || updates::get_all_updates(&pb),
        || packages::get_explicit_packages(&pb),
    );

    let all_updates = match all_updates {
        Ok(v) => v,
        Err(err) => {
            pb.finish_and_clear();
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };
    let explicit_packages = match explicit_packages {
        Ok(v) => v,
        Err(err) => {
            pb.finish_and_clear();
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let common_lines = lines::get_common_lines(&all_updates, &explicit_packages, &pb);
    let update_type_map = UpdateTypeMap::new(&common_lines, &cli.update_type, &pb);

    pb.finish_and_clear();

    update_type_map.print(cli.number_only);
}
