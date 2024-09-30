use super::data::Data;
use crate::print::pretty::Pretty;
use std::{collections::BTreeMap, fmt::Display};

pub enum Kind {
    None,
    Cast,
    Goto,
    Brace,
    Comment,
    Typedef,
    Prototypes,
    LongFunction,
    TooManyFunctions,
}

#[derive(Default)]
struct Stats {
    total: usize,
    goto: usize,
    casts: usize,
    braces: usize,
    typedef: usize,
    comments: usize,
    prototypes: usize,
    long_functions: usize,
    many_functions: usize,
}
impl Stats {
    fn map(&mut self, kind: Kind) {
        match kind {
            Kind::None => (),
            Kind::Goto => self.goto += 1,
            Kind::Cast => self.casts += 1,
            Kind::Brace => self.braces += 1,
            Kind::Typedef => self.typedef += 1,
            Kind::Comment => self.comments += 1,
            Kind::Prototypes => self.prototypes += 1,
            Kind::LongFunction => self.long_functions += 1,
            Kind::TooManyFunctions => self.many_functions += 1,
        }
        self.total += 1;
    }
}
impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Detected {} formatting errors:\n\t{} gotos\n\t{} casts\n\t{} misplaced braces\n\t{} typedefs\n\t{} miswritten comments\n\t{} function prototypes\n\t{} functions with long body\n\t{} files packed with too many functions\n",
             Pretty::fail(&format!("{}", self.total)), self.goto, self.casts, self.braces, self.typedef, self.comments, self.prototypes, self.long_functions, self.many_functions,
        )
    }
}

/// SyntaxError class
#[derive(Default)]
pub struct SyntaxError {
    map: BTreeMap<String, BTreeMap<usize, Vec<String>>>,
    stats: Stats,
}
impl SyntaxError {
    /// Add a new error to the hashmap
    pub fn add(&mut self, data: &Data, kind: Kind, msg: &str) {
        self.stats.map(kind);
        self.map
            .entry(data.get_file())
            .or_default()
            .entry(data.get_line_number())
            .or_default()
            .push(msg.to_string());
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}\n{}",
            {
                let mut out = String::new();
                for (file, lines) in self.map.iter() {
                    out.push_str(&Pretty::info(&format!("\n{:-^80}\n\n", file)));
                    for (line, msgs) in lines {
                        out.push_str(&Pretty::warn(&format!("Line: {line}\n")));
                        for msg in msgs {
                            out.push_str(&format!("  {msg}\n"));
                        }
                    }
                }
                out
            },
            {
                match self.stats.total {
                    0 => {
                        format!(
                            "{}{}",
                            Pretty::succes(&format!("\n{:=^80}\n\n", "STATS")),
                            "No errors found\n"
                        )
                    }
                    _ => {
                        format!(
                            "{}{}",
                            Pretty::fail(&format!("\n{:=^80}\n\n", "STATS")),
                            &self.stats.to_string()
                        )
                    }
                }
            },
            {
                &format!("{} This is not an absolute truth and should only serve as help.\nPlease refer to the 'C Coding Syle' for more info (https://intra.forge.epita.fr/epita-ing-assistants-acu/documents)", Pretty::warn("Warning:"))
            }
        )
    }
}
