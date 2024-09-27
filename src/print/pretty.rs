/// Pretty class made for good-looking results on terminal
pub struct Pretty;
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
    pub fn info(msg: &str) -> String {
        format!("{}{}{}{}", Self::INFO, Self::BOLD, msg, Self::ENDC)
    }

    /// Apply yellow
    pub fn warn(msg: &str) -> String {
        format!("{}{}{}", Self::WARNING, msg, Self::ENDC)
    }

    /// Apply red
    pub fn fail(msg: &str) -> String {
        format!("{}{}{}", Self::FAIL, msg, Self::ENDC)
    }

    /// Apply green
    pub fn succes(msg: &str) -> String {
        format!("{}{}{}", Self::SUCCESS, msg, Self::ENDC)
    }
}
