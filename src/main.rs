mod cli;
mod packages;
mod results;
mod updates;
mod utils;
mod versions;

use clap::Parser;
use spinners::{Spinner, Spinners};

use crate::utils::{fmt, lines};

fn main() {
    let cli = cli::Cli::parse();

    let mut sp = Spinner::new(Spinners::Pong, "Loading".into());

    let (all_updates, explicit_packages) =
        rayon::join(updates::get_all_updates, packages::get_explicit_packages);

    let all_updates = match all_updates {
        Ok(v) => v,
        Err(err) => {
            sp.stop();
            print!("\x1b[2K\r");
            fmt::fatalln("failed to get all updates", Some(&err))
        }
    };
    let explicit_packages = match explicit_packages {
        Ok(v) => v,
        Err(err) => {
            sp.stop();
            print!("\x1b[2K\r");
            fmt::fatalln("failed to get explicit packages", Some(&err))
        }
    };

    let common_lines = lines::get_common_lines(all_updates, explicit_packages);
    let result = results::format_result(&common_lines, &cli.update_type);

    sp.stop();
    print!("\x1b[2K\r");

    results::print_result(result, cli.number);
}
