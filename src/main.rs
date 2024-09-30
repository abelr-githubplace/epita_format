use epita_format::parse::{
    data::{rules, Data},
    syntax::{Kind, SyntaxError},
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use walkdir::WalkDir;

/// Goes through all lines of the current file in data
fn check(filename: &str, data: &mut Data, errors: &mut SyntaxError) -> io::Result<()> {
    // Check file support
    if let Err(e) = data.reset_file(filename) {
        eprintln!("{e}");
        return Ok(());
    }

    // Read file
    let file = File::open(data.get_file())?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        // Check rules
        rules(&line?, data, errors);
    }
    let func_overflow = data.add_func_count_error();
    if func_overflow > 0 {
        errors.add(
            data,
            Kind::TooManyFunctions,
            &format!("Too many functions in this file {}", func_overflow),
        );
    }
    Ok(())
}

/// Goes through the directory recursively and check each files or just check the file
fn go_through(filename: &str, data: &mut Data, errors: &mut SyntaxError) -> io::Result<()> {
    let root = Path::new(&filename);
    // Directory
    if root.is_dir() {
        for entry in WalkDir::new(root) {
            // loops recursively thanks to Walkdir
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // skip directories
                continue;
            }
            let filename = &path.display().to_string();
            check(filename, data, errors)?; // check file
        }
    } else {
        // File
        check(filename, data, errors)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // Initialize data and errors to default values
    let mut data = Data::default();
    let mut errors = SyntaxError::default();

    // Loop through command line arguments
    for arg in std::env::args().collect::<Vec<String>>().iter().skip(1) {
        go_through(arg, &mut data, &mut errors)?;
    }

    // Print the result
    println!("{errors}");

    Ok(())
}
