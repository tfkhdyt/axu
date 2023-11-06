use indicatif::ProgressBar;

pub fn get_common_lines(left: &[String], right: &[String], pb: &ProgressBar) -> Vec<String> {
    pb.set_message("fetch explicit updates");
    let common_lines: Vec<String> = left
        .iter()
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

    common_lines
}
