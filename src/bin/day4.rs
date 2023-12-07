use std::{cell::RefCell, collections::HashMap, fs};

use anyhow::Result;

const BASE: u32 = 2;
const DEFAULT_CARD_COUNT: u32 = 1;
const THIS_CARD_OFFSET: u32 = 1;
const EXCLUSIVE_END_OFFSET: u32 = 1;

type Range = (u32, u32);

fn main() -> Result<()> {
    let problem = aoc::fetch_problem()?;
    let cards = fs::read_to_string(&problem.path)?;

    println!("sum is: {:?}", find_winning_total(&cards));
    println!("total scratchers {:?}", find_total_card_count(&cards));

    Ok(())
}

fn find_total_card_count(cards: &str) -> u32 {
    let mut card_copies: RefCell<HashMap<u32, u32>> = RefCell::new(HashMap::new());
    cards
        .lines()
        .map(|card| {
            let scratcher = card.split(":").collect::<Vec<&str>>();
            let scratcher_id = scratcher
                .get(0)
                .expect("no card title found")
                .split(' ')
                .last()
                .expect("no card id found")
                .parse::<u32>()
                .expect("failed id parse");

            let scratcher_content = scratcher
                .get(1)
                .expect("no num lists in scratcher")
                .split("|")
                .collect::<Vec<&str>>();

            let this_card_count = *card_copies.borrow().get(&scratcher_id).unwrap_or(&1);
            let total_winning_nums = count_total_winning_nums(scratcher_content);
            if total_winning_nums > 0 {
                add_scratcher_copies(
                    card_copies.get_mut(),
                    // don't add yourself, and end of range is exclusive so + 1
                    (scratcher_id + THIS_CARD_OFFSET, scratcher_id + total_winning_nums + EXCLUSIVE_END_OFFSET),
                    this_card_count,
                );
            }

            this_card_count
        })
        .sum()
}

fn add_scratcher_copies(card_copies: &mut HashMap<u32, u32>, range: Range, this_card_count: u32) {
    for scratcher_id in (range.0)..(range.1) {
        if card_copies.contains_key(&scratcher_id) {
            *card_copies.get_mut(&scratcher_id).unwrap() += this_card_count;
        } else {
            // there is always 1 scratcher, so we process based off of this_card_count + 1
            card_copies.insert(scratcher_id, DEFAULT_CARD_COUNT + this_card_count);
        }
    }
}

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

            let total_winning_nums = count_total_winning_nums(game_card_lists);
            if total_winning_nums == 0 {
                return 0;
            }

            BASE.pow(total_winning_nums as u32 - 1)
        })
        .sum()
}

fn count_total_winning_nums(card_content: Vec<&str>) -> u32 {
    let winning_nums = card_content.get(0).unwrap();
    let winning_nums = to_u32_vec(&winning_nums);

    let card_nums = card_content.get(1).unwrap();
    let card_nums = to_u32_vec(&card_nums);

    card_nums
        .iter()
        .filter(|num| winning_nums.contains(num))
        .count() as u32
}

fn to_u32_vec(list: &str) -> Vec<u32> {
    list.split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::find_total_card_count;
    use crate::find_winning_total;

    #[test]
    fn day4_scratchers_example() {
        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let winning_sum = find_winning_total(&s);
        let card_cnt = find_total_card_count(&s);
        assert_eq!(winning_sum, 13, "total not correct: {winning_sum}");
        assert_eq!(card_cnt, 30, "card count incorrect: {card_cnt}");
    }
}
