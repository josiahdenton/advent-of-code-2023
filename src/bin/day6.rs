use anyhow::Result;
use aoc::Part;
use std::fs;

const CHARGE_SPEED: u64 = 1;

fn main() -> Result<()> {
    let problem = aoc::fetch_problem()?;
    let race_info = fs::read_to_string(problem.path)?;

    let (times, distances);
    if problem.part == Part::P1 {
        (times, distances) = parse_as_multiple_races(&race_info);
    } else {
        (times, distances) = parse_as_one_race(&race_info);
    }
    let ways_to_win: u64 = times
        .iter()
        .zip(distances.iter())
        .map(|(time, dist)| find_unique_charging_times(*time, *dist))
        .product();

    println!("ways to win: {}", ways_to_win);

    Ok(())
}

// dist traveled always will peak at the middle of the charging time range
// so let's search for the min no further than the middle
// and the max no less than the middle as it's a bell curve,
// and I only need to find the ends.
fn find_unique_charging_times(time: u64, dist: u64) -> u64 {
    find_max_charge_time(time, dist) - find_min_charge_time(time, dist) + 1
}

fn find_min_charge_time(time: u64, dist_record: u64) -> u64 {
    let mut low = 1;
    let mut high = time / 2; // assume it peaks in the middle
    let mut time_charging;
    // do a modified binary search
    loop {
        time_charging = (low + high) / 2;
        if is_lower_charge_time_bound(time_charging, time, dist_record) {
            break;
        }

        // check which direction to go
        if dist_travel(time_charging, time) <= dist_record {
            // need to go more time charging
            low = time_charging;
        } else {
            // need less time charging
            high = time_charging;
        }
    }

    time_charging
}

fn find_max_charge_time(time: u64, dist_record: u64) -> u64 {
    let mut low = time / 2;
    let mut high = time;
    let mut time_charging;

    loop {
        time_charging = (low + high) / 2;
        if is_upper_charge_time_bound(time_charging, time, dist_record) {
            break;
        }
        // check which direction to go
        if dist_travel(time_charging, time) <= dist_record {
            // need to go with less time charging
            high = time_charging;
        } else {
            // need more time charging
            low = time_charging;
        }
    }

    time_charging
}

fn is_lower_charge_time_bound(time_charging: u64, time: u64, dist_record: u64) -> bool {
    dist_travel(time_charging - 1, time) <= dist_record
        && dist_travel(time_charging, time) > dist_record
}

fn is_upper_charge_time_bound(time_charging: u64, time: u64, dist_record: u64) -> bool {
    dist_travel(time_charging, time) > dist_record
        && dist_travel(time_charging + 1, time) <= dist_record
}

fn dist_travel(time_charging: u64, time: u64) -> u64 {
    time_charging * CHARGE_SPEED * (time - time_charging)
}

// ====================================================
//                      Parsing
// ====================================================

fn parse_as_multiple_races(input: &str) -> (Vec<u64>, Vec<u64>) {
    let input = input.lines().collect::<Vec<&str>>();
    let times = input
        .get(0)
        .expect("no times with title found")
        .split(":")
        .last()
        .expect("no times found");
    let distances = input
        .get(1)
        .expect("no dist with title found")
        .split(":")
        .last()
        .expect("no dist found");
    (
        times
            .split_whitespace()
            .map(|num| num.parse::<u64>().expect("failed to parse time"))
            .collect(),
        distances
            .split_whitespace()
            .map(|num| num.parse::<u64>().expect("failed to parse time"))
            .collect(),
    )
}

fn parse_as_one_race(input: &str) -> (Vec<u64>, Vec<u64>) {
    let input = input.lines().collect::<Vec<&str>>();
    let time = input
        .get(0)
        .expect("no times with title")
        .split(":")
        .last()
        .map(|time| time.replace(" ", ""))
        .expect("no time found");

    let dist = input
        .get(1)
        .expect("no times with title")
        .split(":")
        .last()
        .map(|dist| dist.replace(" ", ""))
        .expect("no time found");

    (
        vec![time.parse::<u64>().expect("failed to parse time")],
        vec![dist.parse::<u64>().expect("failed to parse dist")],
    )
}

#[cfg(test)]
mod test {
    use crate::find_unique_charging_times;

    #[test]
    fn simple_input() {
        assert_eq!(4, find_unique_charging_times(7, 9));
        assert_eq!(8, find_unique_charging_times(15, 40));
        assert_eq!(9, find_unique_charging_times(30, 200));
        assert_eq!(71503, find_unique_charging_times(71530, 940200));
    }
}
