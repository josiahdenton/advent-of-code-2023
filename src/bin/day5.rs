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
            let lowest_location_of_range = map_seed_range_to_lowest_location(seed_range, maps);
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

fn map_seed_range_to_lowest_location(seed_range: Range<i64>, maps: &Vec<Vec<MapEntry>>) -> i64 {
    let mut mapped = vec![seed_range];

    // go through the maps
    for map in &maps[..] {
        mapped = shred(&mut mapped, map);
    }

    // now each range is mapped to a location
    mapped
        .iter()
        // I only care about the lowest value in the range
        .map(|range| range.start)
        .reduce(|lowest_location, location| {
            if location < lowest_location {
                return location;
            }
            lowest_location
        })
        .expect("there should be at least 1 item")
        .clone()
}

#[derive(PartialEq, Eq)]
enum Overlap {
    Right,
    Left,
    Contains,
    None,
}

fn shred(to_map: &mut Vec<Range<i64>>, map: &Vec<MapEntry>) -> Vec<Range<i64>> {
    let mut mapped = vec![];

    // for each seed
    // find an entry with overlap
    // else, we don't map and pass it on as mapped
    // if there are any leftovers, re-shred the leftovers until we have no leftovers (recursive)
    while to_map.len() > 0 {
        let seed_range = to_map.pop().expect("there is a len check, haha");
        // find a map_entry where there is at least 1 overlapping number
        let map_entry = map
            .iter()
            .find(|entry| range_overlap(&seed_range, &entry.src_range) != Overlap::None);
        if let Some(entry) = map_entry {
            // there is at least some overlap, so we split on that mapping
            let overlap = range_overlap(&seed_range, &entry.src_range);
            let (mapped_range, leftover) = split_range(seed_range, entry, overlap);
            mapped.push(mapped_range);
            if let Some(leftover) = leftover {
                to_map.push(leftover);
            }
        } else {
            // no match, we just pass it on as a direct map
            mapped.push(seed_range);
        }
    }

    mapped
}


fn range_overlap(seed_range: &Range<i64>, map_range: &Range<i64>) -> Overlap {
    // remember the end is exclusive, so we deduct 1 to represent the last number in a range
    if map_range.contains(&seed_range.start) && map_range.contains(&(seed_range.end - 1)) {
        return Overlap::Contains;
    } else if map_range.contains(&(seed_range.end - 1)) {
        return Overlap::Right;
    } else if map_range.contains(&seed_range.start) {
        return Overlap::Left;
    }
    Overlap::None
}

fn split_range(
    seed_range: Range<i64>,
    entry: &MapEntry,
    overlap: Overlap,
) -> (Range<i64>, Option<Range<i64>>) {
    match overlap {
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
        Overlap::None => panic!("cannot split a non-overlaping range"),
    }
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
        map_seed_range_to_lowest_location, map_seeds_to_location, parse_seed_header,
        parse_seed_maps, parse_seed_ranges,
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
            println!("mappind seed range {:?}", seed_range);
            let lowest_location_of_range = map_seed_range_to_lowest_location(seed_range, &maps);
            lowest_location_of_ranges.push(lowest_location_of_range);
        }
        let super_low = *lowest_location_of_ranges.iter().min().expect("huh?");
        println!("the lowest low is {}", super_low);

        assert_eq!(46, super_low);
    }
}
