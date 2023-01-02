use regex::Regex;

pub const DOUBLE_WHITE_SPACE: Regex = Regex::new(r"\s\s+").unwrap();
pub const SPECIAL_CHAR: Regex = Regex::new(r"[@#`:\r\n\t\f\v\p{C}]").unwrap();
pub const CHANNEL_MENTION: Regex = Regex::new(r"<#(\d+)>").unwrap();
pub const USER_MENTION: Regex = Regex::new(r"<@!?(\d+)>").unwrap();
pub const ROLE_MENTION: Regex = Regex::new(r"<@&(\d+)>").unwrap();
pub const EVERYONE_MENTION: Regex = Regex::new(r"@everyone").unwrap();
pub const HERE_MENTION: Regex = Regex::new(r"@here").unwrap();

pub fn trim_regex(input: &str, regex: Regex) -> String {
    let result = regex.replace_all(input, "");

    result.trim().to_string()
}