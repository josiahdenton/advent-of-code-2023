use anyhow::Result;
use aoc::Part;
use std::fs;

#[derive(Eq, PartialEq)]
enum Direction {
    Forward,
    Backward,
}

fn main() -> Result<()> {
    let problem = aoc::fetch_problem()?;
    let input = fs::read_to_string(problem.path)?;
    let predictive_sum: i32 = parse(&input)
        .iter()
        .map(|history| {
            extrapolate(
                history,
                if problem.part == Part::P2 {
                    Direction::Backward
                } else {
                    Direction::Forward
                },
            )
        })
        .sum();
    println!("predictive sum: {predictive_sum}");
    Ok(())
}

fn extrapolate(history: &Vec<i32>, direction: Direction) -> i32 {
    // base case
    if history.iter().all(|x| *x == 0) {
        return 0;
    }

    let diff = history
        .iter()
        .zip(history.iter().skip(1))
        .map(|(prev, next)| next - prev)
        .collect::<Vec<i32>>();

    // recursive case
    return if direction == Direction::Forward {
        history
            .iter()
            .last()
            .expect("missing last point in history!")
            + extrapolate(&diff, direction)
    } else {
        history.get(0).expect("missing first point in history!") - extrapolate(&diff, direction)
    };
}

// ====================================================
//                      Parsing
// ====================================================
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("failed to parse history point"))
                .collect()
        })
        .collect()
}

// ====================================================
//                      Unit Tests
// ====================================================
#[cfg(test)]
mod test {
    use crate::{extrapolate, parse, Direction};

    #[test]
    fn day9_forward_case() {
        let s = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let predictive_sum: i32 = parse(&s)
            .iter()
            .map(|history| extrapolate(history, Direction::Forward))
            .sum();
        println!("predictive sum: {predictive_sum}");
    }

    #[test]
    fn day9_backward_case() {
        let s = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let predictive_sum: i32 = parse(&s)
            .iter()
            .map(|history| extrapolate(history, Direction::Backward))
            .sum();
        println!("predictive sum: {predictive_sum}");
    }
}
