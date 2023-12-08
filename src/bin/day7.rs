use std::{cmp::Ordering, collections::HashMap, fs};

use anyhow::Result;

use aoc::Part;

fn main() -> Result<()> {
    let problem = aoc::fetch_problem()?;
    let input = fs::read_to_string(problem.path)?;
    let mut hands = parse(&input, problem.part == Part::P2);
    hands.sort_by(|a, z| {
        a.compare(z)
    });

    // weakest hand is the lowest rank
    let total_winnings: u64 = hands.iter().enumerate().map(|(index, play)| {
        play.bid * ((index + 1) as u64)
    }).sum();

    println!("total winnings is {total_winnings}");
    Ok(())
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn value(&self) -> u32 {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
            Card::Joker => 1,
        }
    }
}

#[derive(Debug)]
struct Play {
    cards: Vec<Card>,
    card_count: HashMap<Card, u32>,
    bid: u64,
}

impl Play {
    fn compare(&self, other: &Self) -> Ordering {
        // if both HighCard, we find which has the highest card value
        let a = self.hand();
        let b = other.hand();

        return if a.value() > b.value() {
            Ordering::Greater
        } else if a.value() < b.value() {
            Ordering::Less
        } else {
            // they are the same value, so find high card
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                if a.value() > b.value() {
                    return Ordering::Greater;
                } else if a.value() < b.value() {
                    return Ordering::Less;
                }
            }
            // this should never happen...
            println!("equal hands (weird...)");
            Ordering::Equal
        };
    }

    fn hand(&self) -> Hand {
        let original = determine_hand(&self.card_count);
        // determine hand without jokers, then do the joker upgrades
        // then compare with previous non wilded hand and see what is
        // the best hand
        let mut hand_upgrades = self.jokers();
        if hand_upgrades == 5 {
            return Hand::FiveKind;
        }
        let no_jokers = self.card_count.clone()
            .iter()
            .filter(|(card, _)| **card != Card::Joker)
            .map(|(card, count)| (card.clone(), *count))
            .collect::<HashMap<Card, u32>>();
        let mut hand = determine_hand(&no_jokers);
        while hand_upgrades > 0 {
            if hand == Hand::FourKind {
                hand = Hand::FiveKind;
            } else if hand == Hand::FullHouse || hand == Hand::ThreeKind {
                hand = Hand::FourKind;
            } else if hand == Hand::TwoPairs {
                hand = Hand::FullHouse;
            } else if hand == Hand::OnePairs {
                hand = Hand::ThreeKind;
            } else if hand == Hand::HighCard {
                hand = Hand::OnePairs;
            }
            hand_upgrades -= 1;
        }

        if original.value() > hand.value() { original } else { hand }
    }

    fn jokers(&self) -> u32 {
        *self.card_count.get(&Card::Joker).unwrap_or(&0)
    }
}

fn determine_hand(card_count: &HashMap<Card, u32>) -> Hand {
    // let's use both card count and num unique cards to determine the hand
    let most_cards = card_count.iter().map(|(_, count)| *count).max().unwrap_or(0);
    return match most_cards {
        5 => Hand::FiveKind,
        4 => Hand::FourKind,
        3 => {
            // also do another code path here for FullHouse
            // Full House means there will ONLY be 2 unique cards
            if card_count.len() == 2 {
                return Hand::FullHouse;
            }
            return Hand::ThreeKind;
        }
        2 => {
            // determine if it's two pair or one pair
            if card_count.iter().map(|(_, count)| *count).filter(|count| *count == 2).collect::<Vec<u32>>().len() == 2 {
                return Hand::TwoPairs;
            }
            return Hand::OnePairs;
        }
        1 => Hand::HighCard,
        0 => Hand::Empty,
        _ => panic!("there must be between 1-5 cards in card_count")
    };
}


#[derive(PartialEq, Eq, Debug)]
enum Hand {
    FiveKind,
    FourKind,
    FullHouse,
    // e.g. 23332
    ThreeKind,
    TwoPairs,
    OnePairs,
    HighCard,
    Empty,
}

impl Hand {
    fn value(&self) -> u32 {
        match self {
            Hand::FiveKind => 7,
            Hand::FourKind => 6,
            Hand::FullHouse => 5,
            Hand::ThreeKind => 4,
            Hand::TwoPairs => 3,
            Hand::OnePairs => 2,
            Hand::HighCard => 1,
            Hand::Empty => 0,
        }
    }
}

// ====================================================
//                      Parsing
// ====================================================
fn parse(input: &str, jack_as_joker: bool) -> Vec<Play> {
    input.lines().map(|line| line_to_hand(line, jack_as_joker)).collect()
}


fn line_to_hand(line: &str, jack_as_joker: bool) -> Play {
    let play = line.split_whitespace().collect::<Vec<&str>>();
    let cards = play.get(0).expect("hand does not exist after split");
    let cards = cards.chars().map(|ch| to_card(ch, jack_as_joker)).collect::<Vec<Card>>();
    let bid = play.get(1).expect("bid does not exist after split");
    let bid = bid.parse::<u64>().expect("bid failed to parse");

    let mut card_count = HashMap::new();
    for card in &cards {
        if let Some(count) = card_count.get(card) {
            card_count.insert(card.clone(), count + 1);
        } else {
            card_count.insert(card.clone(), 1);
        }
    }

    Play { card_count, bid, cards }
}

fn to_card(card: char, jack_as_joker: bool) -> Card {
    match card {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => if jack_as_joker { Card::Joker } else { Card::Jack },
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("not a valid card symbol"),
    }
}

// ====================================================
//                      Unit Tests
// ====================================================
#[cfg(test)]
mod test {
    use crate::parse;

    #[test]
    fn day7_simple_case() {
        let s = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        let mut hands = parse(&s, false);
        hands.sort_by(|a, z| {
            a.compare(z)
        });

        // weakest hand is the lowest rank
        let total_winnings: u64 = hands.iter().enumerate().map(|(index, play)| {
            play.bid * ((index + 1) as u64)
        }).sum();

        println!("total winnings is {total_winnings}");

        assert_eq!(total_winnings, 6440);
    }

    #[test]
    fn day7_simple_case_with_jokers() {
        let s = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        let mut hands = parse(&s, true);
        hands.sort_by(|a, z| {
            a.compare(z)
        });

        // weakest hand is the lowest rank
        let total_winnings: u64 = hands.iter().enumerate().map(|(index, play)| {
            play.bid * ((index + 1) as u64)
        }).sum();

        println!("total winnings is {total_winnings}");

        assert_eq!(total_winnings, 5905);
    }
}
