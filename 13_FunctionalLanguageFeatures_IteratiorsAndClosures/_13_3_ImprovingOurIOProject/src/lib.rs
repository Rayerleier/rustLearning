use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>,)-> Result<Config, &'static str>{
        args.next();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, filename, ignore_case })
    }
}
// we said not to worry about the inefficient clone calls because we would remove them in the future. Well, that time is now!

pub fn search<'a>(query: &str, contents: &'a str)-> Vec<&'a str>{
    contents.lines().filter(|line| line.contains(query)).collect()
}

