use super::config::Config;

#[derive(Default, PartialEq)]
pub enum FTYPE {
    #[default]
    Unsupported,
    C,
    H,
}

pub fn file_type(filename: &str, config: &Config) -> FTYPE {
    if config.is_c_file(filename) {
        return FTYPE::C;
    }
    if config.is_header(filename) {
        return FTYPE::H;
    }
    FTYPE::Unsupported
}
