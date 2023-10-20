use std::{thread, time::Duration};

use indicatif::ProgressBar;
use rayon::prelude::*;

pub fn get_common_lines(left: &[String], right: &[String], pb: &ProgressBar) -> Vec<String> {
    pb.set_message("fetch explicit updates");
    let common_lines: Vec<String> = left
        .par_iter()
        .filter_map(|x| {
            let string_vec = x.split_whitespace().collect::<Vec<&str>>();
            if let Some(first) = string_vec.first() {
                if right.contains(&first.to_string()) {
                    Some(x.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    pb.inc(1);
    thread::sleep(Duration::from_millis(100));

    common_lines
}
