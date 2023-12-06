use std::fmt::Display;

use anyhow::{anyhow, Result};
use aoc::{Part, Pattern};

struct Count {
    reds: u32,
    greens: u32,
    blues: u32,
}

impl Count {
    fn new(reds: u32, greens: u32, blues: u32) -> Self {
        Self {
            reds,
            greens,
            blues,
        }
    }

    fn power(&self) -> u32 {
        self.reds * self.blues * self.greens
    }
}

// for debugging
impl Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R:{} G:{} B:{}", self.reds, self.greens, self.blues,)
    }
}

struct Game {
    id: u32,
    rounds: Vec<Count>,
}

impl Game {
    fn new(id: u32, rounds: Vec<Count>) -> Self {
        Self { id, rounds }
    }

    fn find_smallest_possible_count(&self) -> Count {
        let mut reds = 0;
        let mut blues = 0;
        let mut greens = 0;
        // find the 3 maxes of each color in any round played in a game
        for count in &self.rounds[..] {
            if count.reds > reds {
                reds = count.reds;
            }
            if count.blues > blues {
                blues = count.blues;
            }
            if count.greens > greens {
                greens = count.greens;
            }
        }

        Count::new(reds, greens, blues)
    }

    fn is_possible(&self, rules: &Count) -> bool {
        for round in &self.rounds[..] {
            if round.blues > rules.blues || round.reds > rules.reds || round.greens > rules.greens {
                return false;
            }
        }

        true
    }
}

fn main() {
    let (problem, mut context) = aoc::setup_day();

    let max_cube_rule = Count::new(12, 13, 14);

    let mut sum_ids = 0;

    let mut games = vec![];
    while let Some(line) = context.get_line() {
        let game = parse_game(&line).expect("could not parse game");
        if game.is_possible(&max_cube_rule) {
            sum_ids += game.id;
        }
        games.push(game);
    }
    println!("sum ids is {sum_ids}");

    if problem.part == Part::P1 {
        return;
    }

    let mut sum_powers = 0;
    for game in games {
        let count = game.find_smallest_possible_count();
        sum_powers += count.power();
    }
    println!("sum powers is {sum_powers}");
}

fn game_id(game_header: &str) -> Result<u32> {
    let capture = aoc::search(game_header, vec![Pattern::Number])?;
    let game_id = capture
        .name("number")
        .ok_or(anyhow!("no game id found"))?
        .as_str();
    Ok(game_id.parse()?)
}

fn parse_game(line: &str) -> Result<Game> {
    let split = line.split(":").collect::<Vec<&str>>();
    let game_header = split.get(0).ok_or(anyhow!("no game header"))?;
    let game_rounds = split.get(1).ok_or(anyhow!("no game rounds"))?;
    let id = game_id(&game_header)?;
    let rounds = parse_gameplay(game_rounds)?;

    Ok(Game::new(id, rounds))
}

fn parse_gameplay(game_rounds: &str) -> Result<Vec<Count>> {
    let rounds = game_rounds.split(";").collect::<Vec<&str>>();
    let mut color_counts = vec![];
    for round in rounds {
        color_counts.push(parse_round(round)?);
    }

    Ok(color_counts)
}

fn parse_round(game_round: &str) -> Result<Count> {
    let mut reds = 0;
    let mut blues = 0;
    let mut greens = 0;

    for cube in game_round.split(",") {
        let capture = aoc::search(cube, vec![Pattern::Number, Pattern::Space, Pattern::Word])?;
        let color = capture.name("word").ok_or(anyhow!("no word"))?.as_str();
        let number = capture.name("number").ok_or(anyhow!("no number"))?.as_str();
        match color {
            "blue" => blues = number.parse::<u32>()?,
            "red" => reds = number.parse::<u32>()?,
            "green" => greens = number.parse::<u32>()?,
            _ => {
                return Err(anyhow!(
                    "no color match for cube count in round {game_round}"
                ))
            }
        }
    }

    Ok(Count::new(reds, greens, blues))
}
