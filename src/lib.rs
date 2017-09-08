use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io::{BufReader, Lines};

pub struct Config {
    pub filename: String,
    pub query: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub struct Grep {
    file_stream: Lines<BufReader<File>>,
    query: String,
    case_sensitive: bool,
}

impl Grep {
    pub fn new(config: &Config) -> Result<Self, Box<Error>> {
        let file: BufReader<File> = BufReader::new(File::open(&config.filename)?);
        let file_stream = file.lines();

        Ok(Grep {file_stream, query: config.query.clone(), case_sensitive: config.case_sensitive})
    }
}

impl Iterator for Grep {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line: Option<String> = self.file_stream.next().and_then(|l| l.ok());

            let matched: bool = match line {
                None => return None,
                Some(ref line) => {
                    if self.case_sensitive {
                        line.contains(&self.query)
                    } else {
                        line.to_lowercase().contains(&self.query.to_lowercase())
                    }
                }
            };
            if matched {
                return line;
            }
        }
    }
}
