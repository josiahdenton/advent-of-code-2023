use std::fs;

use anyhow::Result;

const BASE: u32 = 2;

fn main() -> Result<()> {
    let problem = aoc::fetch_problem()?;
    let cards = fs::read_to_string(&problem.path)?;

    println!("sum is: {:?}", find_winning_total(&cards));

    Ok(())
}

// track number of cards 

fn find_winning_total(cards: &str) -> u32 {
    cards
        .lines()
        .map(|card| {
            let game_card_lists = card
                .split(":")
                .last()
                .unwrap()
                .split("|")
                .collect::<Vec<&str>>();

            let winning_nums = game_card_lists.get(0).unwrap();
            let winning_nums = parse_scratch_nums(&winning_nums);

            let card_nums = game_card_lists.get(1).unwrap();
            let card_nums = parse_scratch_nums(&card_nums);

            let total_winning_nums = card_nums
                .iter()
                .filter(|num| winning_nums.contains(num))
                .count();

            if total_winning_nums == 0 {
                return 0;
            }

            BASE.pow(total_winning_nums as u32 - 1)
        })
        .sum()
}

fn parse_scratch_nums(list: &str) -> Vec<u32> {
    list.split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::find_winning_total;

    #[test]
    fn day4_scratchers_examble() {
        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let total = find_winning_total(&s);
        assert!(total == 13, "total not correct: {total}");
    }
}
