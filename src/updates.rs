use std::{
    io,
    sync::{Arc, Mutex},
};

use clap::ValueEnum;
use rayon::prelude::*;

use crate::utils::parsing;

#[derive(ValueEnum, Clone, PartialEq, PartialOrd)]
pub enum UpdateType {
    Major,
    Minor,
    Patch,
    Build,
    Git,
}

pub fn get_all_updates() -> io::Result<Vec<String>> {
    let poison_error_to_io_error =
        |e| io::Error::new(io::ErrorKind::Other, format!("mutex lock error: {:?}", e));

    let updates = Arc::new(Mutex::new(vec![]));
    let commands = [duct::cmd!("checkupdates"), duct::cmd!("yay", "-Qua")];

    commands.par_iter().try_for_each(|cmd| -> io::Result<()> {
        let update = cmd.read()?;
        updates
            .lock()
            .map_err(poison_error_to_io_error)?
            .push(update);
        Ok(())
    })?;

    let all_updates = updates.lock().map_err(poison_error_to_io_error)?.join("\n");
    let mut splited_vec =
        parsing::vec_str_to_vec_string(all_updates.split('\n').collect::<Vec<&str>>());
    splited_vec.sort();

    Ok(splited_vec)
}
