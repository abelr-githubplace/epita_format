use super::{
    config::Config,
    syntax::{Kind, SyntaxError},
};

/// Maximum of functions per file
const MAX_FUNCTIONS_ALLOWED: usize = 10;
/// Maximum of lines per functions
const MAX_LINES_ALLOWED: usize = 25;

/// Create rules functions
macro_rules! rule {
    ($func_name: ident, $check_func: ident $(,$is_other: ident)*) => {
        fn $func_name(&self) -> bool {
            self.config.$check_func(&self.line)
            $(
                || self.config.$is_other(&self.line)
            )*
        }
    };
}

/// Data class
#[derive(Default)]
pub struct Data {
    filename: String,
    line: String,
    line_count: usize,
    func_len: usize,
    func_count: usize,
    in_comment: bool,
    in_func: bool,
    pub config: Config,
}

impl Data {
    pub fn get_file(&self) -> String {
        self.filename.to_owned()
    }

    pub fn reset_file(&mut self, filename: &str) {
        self.filename = filename.to_owned();
        self.line = String::new();
        self.line_count = 1;
        self.func_len = 0;
        self.func_count = 0;
        self.in_comment = false;
        self.in_func = false;
    }

    pub fn set_line(&mut self, line: &str) {
        self.line = line.to_owned();
    }

    pub fn get_line_number(&self) -> usize {
        self.line_count
    }

    pub fn get_func_count(&self) -> usize {
        self.func_count
    }

    pub fn get_func_len(&self) -> usize {
        self.func_len
    }

    pub fn func_line(&mut self) {
        self.func_len += 1;
    }

    pub fn next(&mut self) {
        self.line_count += 1;
    }

    // Calls for rule functions
    rule!(rule_opened_or_closed_brace, is_opened_or_closed_brace);
    rule!(rule_has_brace, has_brace);
    rule!(rule_open_brace, is_opened_brace);
    rule!(rule_closed_brace, is_closed_brace);
    rule!(rule_comment_or_empty, is_comment_or_empty);
    rule!(rule_multi_comment_begin, is_multi_comment_begin);
    rule!(rule_comment_end, is_comment_end);
    rule!(rule_invalid_comment, is_invalid_comment);
    rule!(rule_cast, is_cast);
    rule!(rule_func_proto, is_func_proto);

    /// Rule function: is for misformatted multiline comments
    fn rule_comment(&mut self) -> bool {
        let in_comment = self.rule_multi_comment_begin();

        if self.rule_comment_end() {
            // Comment opened on the same line
            if in_comment {
                return true;
            }

            // Close comment
            self.in_comment = false;
            return !self.rule_invalid_comment();
        }

        self.in_comment |= in_comment;

        if self.in_comment && self.rule_invalid_comment() {
            return false;
        }

        true
    }

    /// Rule function: check for number of functions
    fn rule_func_count(&mut self) -> bool {
        if self.config.is_func_decl(&self.line) {
            self.func_count += 1;
            self.func_len = 0;
            return self.func_count < MAX_FUNCTIONS_ALLOWED;
        }
        true
    }

    /// Rule function: check for length of functions
    fn rule_func_len(&mut self) -> bool {
        if self.rule_open_brace() || self.rule_comment_or_empty() {
            return true;
        }
        if self.rule_closed_brace() {
            self.in_func = false;
            self.func_len = 0;
            return self.func_len <= MAX_LINES_ALLOWED;
        }
        self.func_line();
        true
    }
}

/// Combine all rule is for syntax errors
pub fn rules(data: &mut Data, errors: &mut SyntaxError) -> Result<(), String> {
    if !data.rule_comment() {
        errors.add(data, Kind::Comment, "Misformatted comment");
    }
    if data.rule_has_brace() && !data.rule_opened_or_closed_brace() {
        errors.add(data, Kind::Brace, "All braces must be on their own line.");
    }
    if data.rule_cast() {
        errors.add(data, Kind::Cast, "No casts allowed.");
    }
    if data.rule_func_proto() {
        errors.add(data, Kind::Prototypes, "No function prototypes allowed.");
    }
    if !data.rule_func_len() {
        errors.add(data, Kind::LongFunction, "More than 25 lines in function.");
    }
    if !data.rule_func_count() {
        errors.add(
            data,
            Kind::TooManyFunctions,
            "More than 10 functions in file.",
        );
    }
    Ok(())
}
