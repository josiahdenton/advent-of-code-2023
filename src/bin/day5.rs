use anyhow::{Context, Result};
use std::fs;
use std::ops::Range;

const DEST_POS: usize = 0;
const SRC_POS: usize = 1;
const RANGE_POS: usize = 2;

#[derive(Debug)]
struct MapEntry {
    src_range: Range<i64>,
    dest_offset: i64,
}

impl MapEntry {
    fn new(src_range: Range<i64>, dest_offset: i64) -> Self {
        Self {
            src_range,
            dest_offset,
        }
    }
}

fn main() -> Result<()> {
    let problem = aoc::fetch_problem()?;
    let map_info = fs::read_to_string(problem.path)?;
    let (seeds, maps) = parse_input(&map_info);

    let nearest_seed_location = *map_seeds_to_location(seeds, maps)
        .iter()
        .min()
        .expect("no min?");

    println!("the nearest seed location is: {}", nearest_seed_location);

    Ok(())
}

fn map_seeds_to_location(seeds: Vec<i64>, maps: Vec<Vec<MapEntry>>) -> Vec<i64> {
    let mut locations: Vec<i64> = vec![];
    for seed in seeds {
        let mut mapped_seed = seed;
        for map in &maps[..] {
            mapped_seed = map_seed(mapped_seed, map);
        }
        locations.push(mapped_seed);
    }
    locations
}

fn map_seed(seed: i64, map: &Vec<MapEntry>) -> i64 {
    for entry in map {
        if entry.src_range.contains(&seed) {
            return seed + entry.dest_offset;
        }
    }
    seed
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<MapEntry>>) {
    let lines = input.lines().collect::<Vec<&str>>();
    // first is seeds
    let seeds = parse_seeds(lines.get(0).expect("no seeds in lines"));

    let mut maps: Vec<Vec<MapEntry>> = vec![];
    for line in &lines[1..] {
        println!("{}", line.trim());
        if !(line.trim().len() > 0) {
            println!("{}", line.trim());
            continue;
        }

        if line.contains("map:") {
            maps.push(vec![]);
            continue;
        }
        let latest_map = maps.last_mut().expect("there should be one map always");
        latest_map.push(parse_map_entry(&line));
    }

    println!("{:?}", maps);

    (seeds, maps)
}

fn parse_map_entry(entry: &str) -> MapEntry {
    let entry = entry
        .split(' ')
        .map(|num| num.parse::<i64>().expect("failed to parse map entry"))
        .collect::<Vec<i64>>();
    let (dest_range_0, src_rang_0, range_len) = (
        *entry.get(DEST_POS).expect("no dest"),
        *entry.get(SRC_POS).expect("no src"),
        *entry.get(RANGE_POS).expect("no range"),
    );
    // go calc the offset we need to do....
    let dest_offset = dest_range_0 - src_rang_0;
    let range = src_rang_0..(src_rang_0 + range_len);

    MapEntry::new(range, dest_offset)
}

fn parse_seeds(seeds: &str) -> Vec<i64> {
    seeds
        .split(":")
        .last()
        .expect("no seed input after ':'")
        .split(' ')
        .filter(|seed| seed.len() > 0)
        .inspect(|x| println!(" num to parse : {}", x))
        .map(|seed| seed.trim().parse::<i64>().expect("could not parse seed"))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{parse_input, map_seeds_to_location};

    #[test]
    fn day5_simple_seed_map() {
        let seed_map = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

        let (seeds, maps) = parse_input(&seed_map);

        let nearest_seed_location = *map_seeds_to_location(seeds, maps)
            .iter()
            .min()
            .expect("no min?");

        assert_eq!(35, nearest_seed_location)
    }
}
