use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
};

use clap::{arg, Command};

use anyhow::{anyhow, Result};
use regex::{Captures, Regex};

fn cli() -> Command {
    Command::new("aoc").args([
        arg!(-p --part <part> "part to solve"),
        arg!(-i --input <input> "aoc problem file"),
    ])
}

pub fn fetch_problem() -> Result<Problem> {
    let cmd = cli();
    let matches = cmd.get_matches();
    let part = matches
        .get_one::<String>("part")
        .ok_or(anyhow!("missing part specifier"))?;
    let path = matches
        .get_one::<String>("input")
        .ok_or(anyhow!("missing path specifier"))?;

    let part = match &part[..] {
        "p1" => Part::P1,
        "p2" => Part::P2,
        _ => panic!("please specify p1 or p2"),
    };

    Ok(Problem::new(part, path.to_owned()))
}

pub fn open_into_buffered_reader(path: &str) -> Result<BufReader<File>> {
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

pub fn get_line(reader: &mut BufReader<File>) -> Option<String> {
    let mut line = String::new();
    let bytes_read = reader.read_line(&mut line);
    return match bytes_read {
        Ok(0) => None,
        Ok(_) => Some(line),
        _ => None,
    };
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

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn gcd(a: usize, b: usize) -> usize {
    let mut max = a.max(b);
    let mut min = a.min(b);

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}
