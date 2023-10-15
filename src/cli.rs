use clap::{Parser, Subcommand};
use clap_complete::shells;

use crate::updates::UpdateType;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Show only specified update types
    pub update_type: Vec<UpdateType>,

    /// Print number of updates only
    #[arg(short = 'n', long = "number")]
    pub number_only: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate shell completions
    Completion { shell: shells::Shell },
}
