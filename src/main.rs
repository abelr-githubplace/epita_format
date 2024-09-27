use epita_format::parse::{
    data::{rules, Data},
    syntax::SyntaxError,
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use walkdir::WalkDir;

/// Goes through all lines of the current file in data
fn check(filename: &str, data: &mut Data, errors: &mut SyntaxError) -> io::Result<()> {
    if !data.config.is_supported_file(filename) {
        eprintln!(
            "{} Unsupported filename {filename}",
            epita_format::print::pretty::Pretty::fail("ERROR:")
        );
        return Ok(());
    }
    data.reset_file(filename);
    let file = File::open(data.get_file())?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        data.set_line(&line?);
        if let Err(e) = rules(data, errors) {
            eprintln!("ERROR: {e}");
        }
        data.next();
    }
    Ok(())
}

fn go_through(filename: &str, data: &mut Data, errors: &mut SyntaxError) -> io::Result<()> {
    let root = Path::new(&filename);
    if root.is_dir() {
        for entry in WalkDir::new(root) {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                continue;
            }
            let filename = &path.display().to_string();
            check(filename, data, errors)?;
        }
    } else {
        check(filename, data, errors)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut data = Data::default();
    let mut errors = SyntaxError::default();
    for arg in std::env::args().collect::<Vec<String>>().iter().skip(1) {
        go_through(arg, &mut data, &mut errors)?;
    }
    println!("{errors}");
    Ok(())
}
