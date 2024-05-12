use lazy_static::lazy_static;
use regex::Regex;


pub mod clip;
pub mod auth;


lazy_static!{
    static ref VALID_USERNAME_REGEX: Regex =
        Regex::new("^[[:word:]]{2,32}$").unwrap();
}