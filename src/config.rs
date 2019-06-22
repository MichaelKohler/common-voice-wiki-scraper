use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use toml::value::Array;

pub fn load_config(language: &str) -> Config {
    let file_name = format!("./src/rules/{}.toml", language);
    eprintln!("Loading config at {:?}", file_name);
    // TODO: return default config if file not found..
    let mut file = File::open(file_name).map_err(|e| format!("{}", e)).unwrap();
    let mut config_str = String::new();
    file.read_to_string(&mut config_str)
        .map_err(|e| format!("{}", e)).unwrap();
    let config: Config = toml::from_str(&config_str).unwrap_or_default();
    eprintln!("Using Config {:?}", config);
    config
}

#[derive(Debug,Deserialize)]
pub struct Config {
    pub min_trimmed_length: u16,
    pub min_word_count: u16,
    pub max_word_count: u16,
    pub min_alphanumeric_characters: u16,
    pub may_end_with_colon: bool,
    pub quote_start_with_alphanumeric: bool,
    pub needs_punctuation_end: bool,
    pub needs_uppercase_start: bool,
    pub needs_alphanumeric_start: bool,
    pub disallowed_symbols: Array,
    pub broken_whitespace: Array,
    // FIXME: add abbreviation_patterns...
}

impl Default for Config {
    fn default() -> Config {
        Config {
            min_trimmed_length: 3,
            min_word_count: 1,
            max_word_count: 14,
            min_alphanumeric_characters: 0,
            may_end_with_colon: false,
            quote_start_with_alphanumeric: true,
            needs_punctuation_end: false,
            needs_uppercase_start: false,
            needs_alphanumeric_start: true,
            disallowed_symbols: vec![],
            broken_whitespace: vec![],
        }
    }
}