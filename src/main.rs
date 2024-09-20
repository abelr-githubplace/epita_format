use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use walkdir::WalkDir;

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

struct C;
impl C {
    const _HEADER: &'static str = "\x1b[95m";
    const INFO: &'static str = "\x1b[94m";
    const SUCCESS: &'static str = "\x1b[92m";
    const WARNING: &'static str = "\x1b[93m";
    const FAIL: &'static str = "\x1b[91m";
    const BOLD: &'static str = "\x1b[1m";
    const _UNDERLINE: &'static str = "\x1b[4m";
    const ENDC: &'static str = "\x1b[0m";

    fn info(msg: &str) -> String {
        format!("{}{}{}{}", Self::INFO, Self::BOLD, msg, Self::ENDC)
    }

    fn warn(msg: &str) -> String {
        format!("{}{}{}", Self::WARNING, msg, Self::ENDC)
    }

    fn fail(msg: &str) -> String {
        format!("{}{}{}", Self::FAIL, msg, Self::ENDC)
    }

    fn succes(msg: &str) -> String {
        format!("{}{}{}", Self::SUCCESS, msg, Self::ENDC)
    }
}

#[derive(Default)]
struct Data {
    filename: String,
    line: Option<String>,
    nb_line: usize,
    len_count: usize,
    nb_func: usize,
    nb_err: usize,
    err: HashMap<String, HashMap<String, Vec<String>>>,
    config: Config,
}

fn set_err(data: &mut Data, msg: &str) {
    data.nb_err += 1;
    data.err
        .entry(data.filename.clone())
        .or_default()
        .entry(data.nb_line.to_string())
        .or_default()
        .push(msg.to_string());
}

fn print_moulinette(data: &Data) {
    for (file, lines) in &data.err {
        println!("{}", C::info(&format!("\n{:-^80}\n", file)));
        for (line, msgs) in lines {
            println!("{}", C::warn(&format!("Line: {line}")));
            for msg in msgs {
                println!("  {msg}");
            }
        }
    }
    match data.err.len() {
        0 => {
            println!("{}", C::succes(&format!("\n{:=^80}\n", "STATS")));
            println!("No errors found\n");
        }
        _ => {
            println!("{}", C::fail(&format!("\n{:=^80}\n", "STATS")));
            println!("Total: {} error found\n", C::fail(&data.nb_err.to_string()));
        }
    }
}

fn rule_braces(data: &Data) -> bool {
    match data.line.clone() {
        Some(line) => {
            data.config.check_one(&line)
                || data.config.check_two(&line)
                || data.config.check_three(&line)
                || !data.config.check_four(&line)
        }
        None => panic!("Non-existent line"),
    }
}

fn rule_multi_comments(data: &Data) -> bool {
    match data.line.clone() {
        Some(line) => !data.config.check_multi_comment(&line),
        None => panic!("Non-existent line"),
    }
}

fn rule_no_space_before_comment(data: &Data) -> bool {
    match data.line.clone() {
        Some(line) => !data.config.check_space_before_comment(&line),
        None => panic!("Non-existent line"),
    }
}

fn rule_cast(data: &Data) -> bool {
    match data.line.clone() {
        Some(line) => !data.config.check_no_cast(&line),
        None => panic!("Non-existent line"),
    }
}

fn rule_func_num(data: &mut Data) -> bool {
    match data.line.clone() {
        Some(line) => {
            if data.config.check_is_func_decl(&line) {
                data.nb_func += 1;
                data.nb_func != 11
            } else {
                true
            }
        }
        None => panic!("Non-existent line"),
    }
}

fn rule_func_len(data: &mut Data) -> bool {
    match data.line.clone() {
        Some(line) => {
            if data.config.check_nine(&line) {
                data.len_count = 1;
            } else if data.config.check_ten(&line) {
                data.len_count = 0;
            } else if data.config.check_eleven(&line) {
            } else if data.len_count > 0 {
                data.len_count += 1;
            }
            data.len_count <= 25
        }
        None => panic!("Non-existent line"),
    }
}

fn rules(data: &mut Data) {
    let nb_func_rt = rule_func_num(data);
    let len_func_rt = rule_func_len(data);
    if !rule_multi_comments(data) {
        set_err(data, "Intermediary lines start with ** (5.9)");
    }
    if !rule_no_space_before_comment(data) {
        set_err(data, "Comments must not be indented (5.9 bis)");
    }
    if !rule_braces(data) {
        set_err(data, "All braces MUST be on their own line. (6.1)");
    }
    if !rule_cast(data) {
        set_err(data, "No casts allowed. (8.1)");
    }
    if !nb_func_rt {
        set_err(data, "More than 10 functions in file. (8.9)");
    }
    if !len_func_rt {
        set_err(data, "More than 25 lines in function. (8.10)");
    }
}

fn readlines(data: &mut Data) -> io::Result<()> {
    data.len_count = 0;
    data.nb_func = 0;
    data.nb_line = 1;
    let file = File::open(data.filename.clone())?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        data.line = line.ok();
        rules(data);
        data.nb_line += 1;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut data = Data::default();
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
                    readlines(&mut data)?;
                }
            }
        } else if data.config.check_file(arg) {
            data.filename = arg.clone();
            readlines(&mut data)?;
        }
    }
    print_moulinette(&data);
    Ok(())
}
