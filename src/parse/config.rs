use regex::Regex;

/// Raw string for types
const TYPE: &str = r"((const|void|char|short|int|long|usize_t|ssize_t|float|double|\*)( )?)+";
/// Raw string for identifiers
const IDENT: &str = r"([a-zA-Z_][a-zA-Z0-9_]*)";
/// Raw string for any number of spaces
const ANY_SPACE: &str = r"[ ]*";
/// Raw string for opening curly brace
const OPENED_BRACE_LINE: &str = r"^\{$";
/// Raw string for closing curly brace
const CLOSED_BRACE_LINE: &str = r"^}( //.*)?$";
/// Raw string for brace
const HAS_BRACE: &str = r"^.*[{}].*$";
/// Raw string for opening multiline comment
const MULTI_COMMENT_BEGIN: &str = r"/\*(\*)?";
/// Raw string for closing comment
const COMMENT_END: &str = r"\*/";
/// Raw string for goto statement
const GOTO: &str = r"^.*(goto) .*$";
/// Raw string for typedef statement
const TYPEDEF: &str = r"^typedef (struct|union).*$";
/// Raw string for 80 char per line
const LINE_80: &str = r"^.{0,80}$";
/// Raw string for single line comment
const SINGLE_LINE_COMMENT: &str = r"( //.*)?";

/// Create the Config class from fields, methods name and regex string
macro_rules! config {
    ($($field: ident, $method: ident, $reg: expr),*) => {
        pub struct Config {
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
    c_file,
    is_c_file,
    r"^(\.)?[a-z_/]+\.c$",
    header,
    is_header,
    r"^(\.)?[a-z_/]+\.h$",
    multi_comment_begin,
    is_multi_comment_begin,
    MULTI_COMMENT_BEGIN,
    comment_end,
    is_comment_end,
    COMMENT_END,
    comment_or_empty,
    is_comment_or_empty,
    &format!("^{ANY_SPACE}(((/\\*|\\*\\*).*)|(\\*/)|(//.*)|)$"),
    invalid_comment,
    is_invalid_comment,
    &format!("^({ANY_SPACE}((/\\*(\\*)?[^*]+)|(\\*/.+))|([ ]+((/\\*(\\*)?)|(\\*/)|(\\*\\*.*))))$"),
    cast,
    is_cast,
    &format!("^.*\\([ ]*(unsigned[ ]+|signed[ ]+)?{TYPE}\\).*$"),
    func_decl,
    is_func_decl,
    &format!("^(inline )?(static )?{TYPE}{IDENT}\\(.*\\){SINGLE_LINE_COMMENT}$"),
    func_proto,
    is_func_proto,
    &format!("^(inline )?(static )?{TYPE}{IDENT}\\(.*\\);{SINGLE_LINE_COMMENT}$"),
    opened_brace,
    is_opened_brace,
    OPENED_BRACE_LINE,
    closed_brace,
    is_closed_brace,
    CLOSED_BRACE_LINE,
    open_or_close_brace,
    is_opened_or_closed_brace,
    &format!("^{ANY_SPACE}[{{}}]( while{ANY_SPACE}\\(.*\\))?(;)?({ANY_SPACE}//(/)? .*)?({ANY_SPACE}/\\* .* \\*/)?$"),
    has_brace,
    has_brace,
    HAS_BRACE,
    goto,
    is_goto,
    GOTO,
    typedef,
    is_typedef,
    TYPEDEF,
    line_80,
    is_line_80,
    LINE_80
);
