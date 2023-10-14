use clap::Parser;

use crate::updates::UpdateType;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Show only specified update type
    pub update_type: Option<Vec<UpdateType>>,

    /// Show number of updates only
    #[arg(short, long)]
    pub number: bool,
}
