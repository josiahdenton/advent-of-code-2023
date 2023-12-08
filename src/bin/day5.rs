use anyhow::Result;
use aoc::Part;
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
    let seeds = parse_seed_header(&map_info);
    let maps = &parse_seed_maps(&map_info);

    if problem.part == Part::P1 {
        let nearest_seed_location = *map_seeds_to_location(seeds, maps)
            .iter()
            .min()
            .expect("no min?");
        println!("the nearest seed location is: {}", nearest_seed_location);
    } else {
        let mut lowest_location_of_ranges = vec![];
        let seed_ranges = parse_seed_ranges(&seeds);
        for seed_range in seed_ranges {
            let lowest_location_of_range = *map_seed_ranges_to_location(seed_range, maps)
                .iter()
                .min()
                .expect("no min??");
            lowest_location_of_ranges.push(lowest_location_of_range);
        }
        let super_low = lowest_location_of_ranges.iter().min().expect("huh?");
        println!("the lowest low is {}", super_low);
    }

    Ok(())
}

fn map_seeds_to_location(seeds: Vec<i64>, maps: &Vec<Vec<MapEntry>>) -> Vec<i64> {
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

// were going to have to do ranges... that means we
// want to split the ranges if they don't fit in the range of the map.
// if there is no map range it fits in, we just map directly
// this will happen for every range
fn map_seed_ranges_to_location(seed_range: Range<i64>, maps: &Vec<Vec<MapEntry>>) -> Vec<i64> {
    let mut thousandth_thresh = (seed_range.end - seed_range.start) / 1000;
    let start_point_offset = seed_range.start;
    let mut locations: Vec<Range<i64>> = vec![];

    let mut to_map = vec![seed_range];
    let mut mapped = vec![];

    while to_map.len() > 0 {
        for map in &maps[..] {
            // compare the ranges with the MapEntry and see where we need to split...
            // if any overlap found, split it
            let (mapped_range, leftovers) = shred_ranges(seed_range, map);
            mapped
        }

    }

    // would be easy to turn this into a map & min
    vec![]
}

enum Overlap {
    Right,
    Left,
    Contains,
    None,
}

fn shred_ranges(seed_range: Range<i64>, map: &Vec<MapEntry>) -> (Range<i64>, Option<Range<i64>>) {
    for entry in map {
        // I want to compare ranges and see if they overlap at all
        let overlap = range_overlap(&seed_range, &entry.src_range);
        // if overlap == Overlap::None {
        //     continue;
        // }

        return match overlap {
            Overlap::Right => (
                (entry.src_range.start + entry.dest_offset)..(seed_range.end + entry.dest_offset),
                Some(seed_range.start..entry.src_range.start),
            ),
            Overlap::Left => (
                (seed_range.start + entry.dest_offset)..(entry.src_range.end + entry.dest_offset),
                Some(entry.src_range.end..seed_range.end),
            ),
            Overlap::Contains => (
                (seed_range.start + entry.dest_offset)..(seed_range.end + entry.dest_offset),
                None,
            ),
            Overlap::None => continue,
        };
    }

    // return range i64 where range is the mapped offset, second arg is any leftovers that did not
    // fit, None if everything "mapped"
    (seed_range, None)
}

fn range_overlap(seed_range: &Range<i64>, map_range: &Range<i64>) -> Overlap {
    if map_range.contains(&seed_range.end) {
        return Overlap::Right;
    } else if map_range.contains(&seed_range.start) {
        return Overlap::Left;
    } else if map_range.contains(&seed_range.start) && map_range.contains(&seed_range.end) {
        return Overlap::Contains;
    }

    Overlap::None
}

fn map_seed(seed: i64, map: &Vec<MapEntry>) -> i64 {
    for entry in map {
        if entry.src_range.contains(&seed) {
            return seed + entry.dest_offset;
        }
    }
    seed
}

fn parse_seed_header(input: &str) -> Vec<i64> {
    let lines = input.lines().collect::<Vec<&str>>();
    parse_seeds(lines.get(0).expect("no seeds in lines"))
}

fn parse_seed_maps(input: &str) -> Vec<Vec<MapEntry>> {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut maps: Vec<Vec<MapEntry>> = vec![];
    for line in &lines[1..] {
        if !(line.trim().len() > 0) {
            continue;
        }

        if line.contains("map:") {
            maps.push(vec![]);
            continue;
        }
        let latest_map = maps.last_mut().expect("there should be one map always");
        latest_map.push(parse_map_entry(&line));
    }

    maps
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

fn parse_seed_ranges(seed_ranges: &Vec<i64>) -> Vec<Range<i64>> {
    let start = seed_ranges
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, seed_range)| *seed_range);
    let offset = seed_ranges
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, seed_range)| *seed_range);

    // iterate through the seeds, creating ranges
    start
        .zip(offset)
        .map(|(start, offset)| start..(start + offset))
        .collect()
}

fn parse_seeds(seeds: &str) -> Vec<i64> {
    seeds
        .split(":")
        .last()
        .expect("no seed input after ':'")
        .split(' ')
        .filter(|seed| seed.len() > 0)
        .map(|seed| seed.trim().parse::<i64>().expect("could not parse seed"))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{
        map_seed_ranges_to_location, map_seeds_to_location, parse_seed_header, parse_seed_maps,
        parse_seed_ranges,
    };

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

        let seeds = parse_seed_header(&seed_map);
        let maps = parse_seed_maps(&seed_map);

        let nearest_seed_location = *map_seeds_to_location(seeds, &maps)
            .iter()
            .min()
            .expect("no min?");

        assert_eq!(35, nearest_seed_location)
    }

    #[test]
    fn day5_simple_seed_map_using_ranges() {
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

        let seeds = parse_seed_header(&seed_map);
        let maps = parse_seed_maps(&seed_map);

        let mut lowest_location_of_ranges = vec![];
        let seed_ranges = parse_seed_ranges(&seeds);
        for seed_range in seed_ranges {
            let lowest_location_of_range = *map_seed_ranges_to_location(seed_range, &maps)
                .iter()
                .min()
                .expect("no min??");
            lowest_location_of_ranges.push(lowest_location_of_range);
        }
        let super_low = *lowest_location_of_ranges.iter().min().expect("huh?");
        println!("the lowest low is {}", super_low);

        assert_eq!(46, super_low);
    }
}
