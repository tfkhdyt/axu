pub fn get_common_lines(left: Vec<String>, right: Vec<String>) -> Vec<String> {
    let mut common_lines: Vec<String> = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i].contains(&right[j]) {
            common_lines.push(left[i].to_owned());
            i += 1;
            j += 1;
        } else if left[i] < right[j] {
            i += 1;
        } else {
            j += 1;
        }
    }

    common_lines
}
