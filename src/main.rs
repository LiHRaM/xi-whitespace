extern crate xi_core_lib;
extern crate xi_rope;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use xi_core_lib::internal::line_ending::*;
use xi_core_lib::internal::whitespace::*;
use xi_rope::Rope;

fn main() -> Result<(), ::std::io::Error> {
    let args: Vec<String> = ::std::env::args().collect();

    if args.len() != 2 {
        println!("Missing file parameter!");
        return Ok(())
    }

    let path = &args[1];

    println!("{}", path);

    let path = PathBuf::from(path);

    assert!(path.exists());

    let mut file = File::open(PathBuf::from(path)).expect("file not found!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file!");

    let rope = Rope::from(contents);
    let indent = Indentation::parse(&rope);
    let line_ending = LineEnding::parse(&rope);

    println!(
        r#"
    Indentation: {:?}
    Line endings: {:?}
    "#,
        indent, line_ending
    );

    Ok(())
}
