use rayon::prelude::*;

pub fn vec_str_to_vec_string(vec: Vec<&str>) -> Vec<String> {
    vec.par_iter()
        .map(|&str| String::from(str))
        .collect::<Vec<String>>()
}
