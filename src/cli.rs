use clap::{Parser, Subcommand};
use clap_complete::shells;

use crate::updates::UpdateType;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Show only specified update type
    pub update_type: Option<Vec<UpdateType>>,

    /// Show number of updates only
    #[arg(short, long)]
    pub number: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate shell completions
    Completion { shell: shells::Shell },
}
