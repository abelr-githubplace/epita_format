use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use walkdir::WalkDir;

/// Maximum of functions per file
const MAX_FUNCTIONS_ALLOWED: usize = 10;
/// Maximum of lines per functions
const MAX_LINES_ALLOWED: usize = 25;

/// Fail message
const FAIL: &str = "Non-existent line";

/// Create rules functions
///
/// WARNING: check `!first_check` but `following_checks`
macro_rules! rule {
    ($func_name: ident, $check_func: ident $(,$check_other: ident)*) => {
        fn $func_name(data: &Data) -> Result<bool, String> {
            match data.line.clone() {
                Some(line) => {Ok(!data.config.$check_func(&line) // !first_check
                    $(
                        || data.config.$check_other(&line) // following_checks
                    )*
                )},
                None => Err(FAIL.to_string()),
            }
        }
    };
}

/// Create the Config class from fields, methods name and regex string
macro_rules! config {
    ($($field: ident, $method: ident, $reg: literal),*) => {
        struct Config {
            $(
                $field : Option<Regex>,
            )*
        }
        impl Config {
            $(
                pub fn $method(&self, line: &str) -> bool {
                    self.$field.as_ref().is_some_and(|reg| reg.is_match(line))
                }
            )*
        }
        impl Default for Config {
            fn default() -> Self {
                Config {
                    $(
                        $field: Regex::new($reg).ok(),
                    )*
                }
            }
        }
    };
}

// Call for Config class creation
config!(
    file,
    check_file,
    r"^.+\.(c|h)$",
    ignore,
    check_ignore,
    r"^.*((.git/|build/).*|(.git|build))$",
    one,
    check_one,
    r"^([ ]*(\{|})(,|;)?)$",
    two,
    check_two,
    r#"^.*(["']){1}.*(\{|}).*(["']){1}.*$"#,
    three,
    check_three,
    r"^[ ]*(\{|})(,|;)?[ ]*\\",
    four,
    check_four,
    r"^.*(\{|}).*$",
    multi_comment,
    check_multi_comment,
    r"^(([ ]?\*[ ])|([ ]?\*$))",
    space_before_comment,
    check_space_before_comment,
    r"^([ ]+[\*]+.*)",
    no_cast,
    check_no_cast,
    r"^.*=[ ]*[(].+([ ][*])?[)].+$",
    is_func_decl,
    check_is_func_decl,
    r"^(void|static|int|short|char|struct|long|inline)([^=]+[(][^=]*)$",
    nine,
    check_nine,
    r"^(void|static|int|short|char|struct|long|inline)([^=]+[(][^=]*)$",
    ten,
    check_ten,
    r"^}$",
    eleven,
    check_eleven,
    r"^([ ]*(\{|})|[ ]*|[ ]*//.*)$"
);

/// Pretty class made for good-looking results on terminal
struct Pretty;
impl Pretty {
    /// Magenta
    const _HEADER: &'static str = "\x1b[35m";
    /// Blue
    const INFO: &'static str = "\x1b[34m";
    /// Green
    const SUCCESS: &'static str = "\x1b[32m";
    /// Yellow
    const WARNING: &'static str = "\x1b[33m";
    /// Red
    const FAIL: &'static str = "\x1b[31m";
    /// Bold
    const BOLD: &'static str = "\x1b[1m";
    /// Underline
    const _UNDERLINE: &'static str = "\x1b[4m";
    /// Reset
    const ENDC: &'static str = "\x1b[0m";

    /// Apply blue and bold
    fn info(msg: &str) -> String {
        format!("{}{}{}{}", Self::INFO, Self::BOLD, msg, Self::ENDC)
    }

    /// Apply yellow
    fn warn(msg: &str) -> String {
        format!("{}{}{}", Self::WARNING, msg, Self::ENDC)
    }

    /// Apply red
    fn fail(msg: &str) -> String {
        format!("{}{}{}", Self::FAIL, msg, Self::ENDC)
    }

    /// Apply green
    fn succes(msg: &str) -> String {
        format!("{}{}{}", Self::SUCCESS, msg, Self::ENDC)
    }
}

/// Data class
#[derive(Default)]
struct Data {
    filename: String,
    line: Option<String>,
    nb_line: usize,
    len_count: usize,
    nb_func: usize,
    config: Config,
}

/// SyntaxError class
#[derive(Default)]
struct SyntaxError {
    map: HashMap<String, HashMap<usize, Vec<String>>>,
    error_count: usize,
}
impl SyntaxError {
    /// Add a new error to the hashmap
    pub fn add(&mut self, data: &Data, msg: &str) {
        self.error_count += 1;
        self.map
            .entry(data.filename.clone())
            .or_default()
            .entry(data.nb_line)
            .or_default()
            .push(msg.to_string());
    }

    /// Print results onto the terminal
    fn pretty_print(&self) {
        for (file, lines) in self.map.iter() {
            println!("{}", Pretty::info(&format!("\n{:-^80}\n", file)));
            for (line, msgs) in lines {
                println!("{}", Pretty::warn(&format!("Line: {line}")));
                for msg in msgs {
                    println!("  {msg}");
                }
            }
        }
        match self.error_count {
            0 => {
                println!("{}", Pretty::succes(&format!("\n{:=^80}\n", "STATS")));
                println!("No errors found\n");
            }
            _ => {
                println!("{}", Pretty::fail(&format!("\n{:=^80}\n", "STATS")));
                println!(
                    "Total: {} error found\n",
                    Pretty::fail(&self.error_count.to_string())
                );
            }
        }
    }
}

// Calls for rule functions
rule!(rule_braces, check_four, check_one, check_two, check_three);
rule!(rule_multi_comment, check_multi_comment);
rule!(rule_no_space_before_comment, check_space_before_comment);
rule!(rule_cast, check_no_cast);

/// Rule function: check for number of functions
fn rule_func_num(data: &mut Data, line: &str) -> bool {
    if data.config.check_is_func_decl(line) {
        data.nb_func += 1;
        return data.nb_func < MAX_FUNCTIONS_ALLOWED;
    }
    true
}

/// Rule function: check for length of functions
fn rule_func_len(data: &mut Data, line: &str) -> bool {
    if data.config.check_nine(line) {
        data.len_count = 1;
    } else if data.config.check_ten(line) {
        data.len_count = 0;
    } else if data.config.check_eleven(line) {
    } else if data.len_count > 0 {
        data.len_count += 1;
    }
    data.len_count <= MAX_LINES_ALLOWED
}

/// Combine the two above functions for error handling
fn rule_func(data: &mut Data) -> Result<(bool, bool), String> {
    match data.line.clone() {
        Some(line) => Ok((rule_func_len(data, &line), rule_func_num(data, &line))),
        None => Err(FAIL.to_string()),
    }
}

/// Combine all rule check for syntax errors
fn rules(data: &mut Data, errors: &mut SyntaxError) {
    if rule_multi_comment(data).is_ok_and(|r| !r) {
        errors.add(data, "Intermediary lines start with ** (5.9)");
    }
    if rule_no_space_before_comment(data).is_ok_and(|r| !r) {
        errors.add(data, "Comments must not be indented (5.9 bis)");
    }
    if rule_braces(data).is_ok_and(|r| !r) {
        errors.add(data, "All braces MUST be on their own line. (6.1)");
    }
    if rule_cast(data).is_ok_and(|r| !r) {
        errors.add(data, "No casts allowed. (8.1)");
    }
    if let Ok((a, b)) = rule_func(data) {
        // rule_func_len
        if !a {
            errors.add(data, "More than 25 lines in function. (8.10)");
        }
        // rule_func_num
        if !b {
            errors.add(data, "More than 10 functions in file. (8.9)");
        }
    }
}

/// Goes through all lines of the current file in data
fn readlines(data: &mut Data, errors: &mut SyntaxError) -> io::Result<()> {
    data.len_count = 0;
    data.nb_func = 0;
    data.nb_line = 1;
    let file = File::open(data.filename.clone())?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        data.line = line.ok();
        rules(data, errors);
        data.nb_line += 1;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut data = Data::default();
    let mut errors = SyntaxError::default();
    let args: Vec<String> = std::env::args().collect();
    for arg in args.iter().skip(1) {
        let path = Path::new(&arg);
        if path.is_dir() {
            for entry in WalkDir::new(path) {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() && data.config.check_ignore(&path.display().to_string()) {
                    continue;
                }
                if data.config.check_file(&path.display().to_string()) {
                    data.filename = path.display().to_string();
                    readlines(&mut data, &mut errors)?;
                }
            }
        } else if data.config.check_file(arg) {
            data.filename = arg.clone();
            readlines(&mut data, &mut errors)?;
        }
    }
    errors.pretty_print();
    Ok(())
}
