use std::{error::Error, process};

pub fn fatalln(message: &str, err: Option<&dyn Error>) -> ! {
    eprintln!("Error: {}", message);
    if let Some(err) = err {
        eprintln!("{}", err);
    }
    process::exit(1)
}
