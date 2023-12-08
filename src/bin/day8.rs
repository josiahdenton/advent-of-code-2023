use anyhow::Result;
use aoc::{lcm, Part};
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    let problem = aoc::fetch_problem()?;
    let s = fs::read_to_string(&problem.path)?;
    let (move_list, map) = parse(&s);
    let mut steps: usize = 0;
    if problem.part == Part::P1 {
        steps = walk(&map, &move_list, ORIGIN, &|location| {
            location.chars().all(|ch| ch == 'Z')
        });
    } else if problem.part == Part::P2 {
        steps = all_origins(&map)
            .iter()
            .map(|origin| {
                walk(&map, &move_list, origin, &|location| {
                    location.chars().last().is_some_and(|ch| ch == 'Z')
                })
            })
            .reduce(|acc, steps| lcm(acc, steps))
            .expect("failed to get lcm for all origins");
    }
    println!("steps: {}", steps);

    Ok(())
}

const ORIGIN: &'static str = "AAA";

fn all_origins(map: &HashMap<String, Location>) -> Vec<String> {
    map.iter()
        .map(|(origin, _)| origin.clone())
        .filter(|origin| {
            origin
                .chars()
                .last()
                .and_then(|ch| Some(ch == 'A'))
                .unwrap_or(false)
        })
        .collect()
}

fn walk(
    map: &HashMap<String, Location>,
    move_list: &Vec<Move>,
    location: &str,
    is_destination: &dyn Fn(&str) -> bool,
) -> usize {
    let mut steps_taken = 0;
    let mut current_location = location.to_string();
    while !is_destination(&current_location) {
        current_location = take_step(&map, &move_list, steps_taken, &current_location);
        steps_taken += 1;
    }

    steps_taken
}

fn take_step(
    map: &HashMap<String, Location>,
    move_list: &Vec<Move>,
    steps_taken: usize,
    current_location: &String,
) -> String {
    move_list
        .get(steps_taken % move_list.len())
        .and_then(|mv| {
            map.get(current_location).and_then(|options| {
                if *mv == Move::Left {
                    Some(options.0.clone())
                } else {
                    Some(options.1.clone())
                }
            })
        })
        .expect(&format!(
            "failed to get next location, current: {}",
            current_location
        ))
}

#[derive(Debug, Eq, PartialEq)]
enum Move {
    Left,
    Right,
}

impl Move {
    fn new(ch: char) -> Self {
        if ch == 'L' {
            Move::Left
        } else {
            Move::Right
        }
    }
}

type Location = (String, String);

// ====================================================
//                      Parsing
// ====================================================
fn parse(input: &str) -> (Vec<Move>, HashMap<String, Location>) {
    let lines: Vec<&str> = input.lines().collect();
    let move_list = lines.get(0).expect("no move list").to_string();

    let mut moves = vec![];
    for ch in move_list.chars() {
        moves.push(Move::new(ch));
    }

    let mut map = HashMap::new();
    for line in &lines[2..] {
        let location_key_map: Vec<&str> = line.split("=").collect();
        let origin = location_key_map
            .get(0)
            .expect("key missing")
            .trim()
            .to_string();
        let travel_options = location_key_map
            .get(1)
            .expect("travel options missing")
            .replace("(", "")
            .replace(")", "")
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        map.insert(
            origin,
            (
                travel_options
                    .get(0)
                    .expect("missing first travel option")
                    .trim()
                    .to_string(),
                travel_options
                    .get(1)
                    .expect("missing second travel option")
                    .trim()
                    .to_string(),
            ),
        );
    }

    (moves, map)
}

// ====================================================
//                      Unit Tests
// ====================================================
#[cfg(test)]
mod test {
    use aoc::lcm;
    use crate::{parse, walk, ORIGIN, all_origins};

    #[test]
    fn day8_two_move() {
        let s = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let (move_list, map) = parse(&s);
        let steps = walk(&map, &move_list, ORIGIN, &|location| {
            location.chars().all(|ch| ch == 'Z')
        });
        println!("steps: {}", steps);
    }

    #[test]
    fn day8_six_step() {
        let s = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let (move_list, map) = parse(&s);
        let steps = walk(&map, &move_list, ORIGIN, &|location| {
            location.chars().all(|ch| ch == 'Z')
        });
        println!("steps: {}", steps);
    }

    #[test]
    fn day8_multi_walk() {
        let s = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let (move_list, map) = parse(&s);
        let steps = all_origins(&map)
            .iter()
            .map(|origin| {
                walk(&map, &move_list, origin, &|location| {
                    location.chars().last().is_some_and(|ch| ch == 'Z')
                })
            })
            .reduce(|acc, steps| lcm(acc, steps))
            .expect("failed to get lcm for all origins");

        println!("steps: {}", steps);
    }
}
