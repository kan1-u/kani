use clap::*;
use kani_evaluator::{nom, Evaluator};
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let matches = clap_app!(kani =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg file: -f --file [PATH] +takes_value "Path of the source file")
        (@arg code: -c --code [CODE] +takes_value "Code you want to run inline")
    )
    .get_matches();

    let path = matches.value_of("file");
    let code = matches.value_of("code");
    let code = match (path, code) {
        (Some(path), _) => read_file(path).ok(),
        (_, Some(code)) => Some(code.to_owned()),
        _ => None,
    };

    if let Some(code) = code {
        match Evaluator::new().eval_code(&code) {
            Ok(object) => println!("{}", object),
            Err(nom::Err::Error(_)) => println!("Parser error"),
            Err(nom::Err::Failure(_)) => println!("Parser failure"),
            Err(nom::Err::Incomplete(_)) => println!("Incomplete parsing"),
        }
    }
}
