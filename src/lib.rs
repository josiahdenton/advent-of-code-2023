use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
};

use clap::{arg, Command};

use anyhow::{anyhow, Result};
use regex::{Captures, Regex};

fn cli() -> Command {
    Command::new("aoc").args([
        arg!(-p --part <PART> "part to solve"),
        arg!(-i --input <input> "aoc problem file"),
    ])
}

pub fn setup_day() -> (Problem, AocContext) {
    let problem = fetch_problem();
    let reader = open_into_buffered_reader(&problem.path).expect("failed to get input file");
    let context = AocContext::new(reader);
    (problem, context)
}

// NOTE: improvement idea - more Result<...> usage
fn fetch_problem() -> Problem {
    let cmd = cli();
    let matches = cmd.get_matches();
    let part = matches
        .get_one::<String>("part")
        .expect("missing part specifier");
    let path = matches
        .get_one::<String>("input")
        .expect("missing part specifier");

    let part = match &part[..] {
        "p1" => Part::P1,
        "p2" => Part::P2,
        _ => panic!("please specify p1 or p2"),
    };

    Problem::new(part, path.to_owned())
}

fn open_into_buffered_reader(path: &str) -> Result<BufReader<File>> {
    let fp = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(fp);

    Ok(reader)
}

pub enum Pattern {
    Space,  // \s+
    Number, // \d+
    Word,   // \w+
}

impl Pattern {
    pub fn into_value<'a>(self) -> &'a str {
        match self {
            Pattern::Space => r"\s+",
            Pattern::Number => r"(?<number>\d+)",
            Pattern::Word => r"(?<word>\w+)",
        }
    }
}

pub fn search(line: &str, patterns: Vec<Pattern>) -> Result<Captures> {
    let mut expect = String::new();
    for pattern in patterns {
        expect.push_str(pattern.into_value());
    }

    let re = Regex::new(&expect)?;
    Ok(re
        .captures(line)
        .ok_or(anyhow!("failed to capture on pattern {expect}"))?)
}

pub struct AocContext {
    reader: BufReader<File>,
}

// TODO: setup logging to a file
impl AocContext {
    pub fn new(reader: BufReader<File>) -> Self {
        Self { reader }
    }

    // NOTE: improvement idea, make this an iterator instead
    pub fn get_line(&mut self) -> Option<String> {
        let mut line = String::new();
        let bytes_read = self.reader.read_line(&mut line);
        match bytes_read {
            Ok(0) => None,
            Ok(_) => Some(line),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Part {
    P1,
    P2,
}

pub struct Problem {
    pub part: Part,
    pub path: String,
}

impl Problem {
    fn new(part: Part, path: String) -> Self {
        Self { part, path }
    }
}
