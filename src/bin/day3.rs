use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

use anyhow::Result;

type Point = (usize, usize);
type Range = (usize, usize);

type PartNumber = Rc<RefCell<u32>>;

fn main() -> Result<()> {
    let problem = aoc::fetch_problem()?;
    let schematic = fs::read_to_string(&problem.path)?;

    println!("sum is: {:?}", sum_engine_schematic(&schematic));

    Ok(())
}

fn sum_engine_schematic(schematic: &str) -> (u32, u32) {
    let mut point_map: HashMap<Point, PartNumber> = HashMap::new();
    let mut x_start = None;
    let mut num = String::new();

    let mut parsed_schematic = vec![];
    for (y, line) in schematic.lines().filter(|s| s.len() > 0).enumerate() {
        let mut parsed_line = vec![];
        for (x, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                num.push(ch);
                // start tracking
                if None == x_start {
                    x_start = Some(x);
                }
            } else {
                x_start = x_start.and_then(|x_start| {
                    let value = num.parse::<u32>().expect(&format!("bad num val: {num}"));
                    add_to_point_map(&mut point_map, (x_start, x), y, value);
                    num.clear();

                    None
                });
            }
            parsed_line.push(ch);
        }
        // repeat opp for end
        if num.len() > 0 {
            x_start = x_start.and_then(|x_start| {
                let value = num.parse().expect(&format!("bad num val: {num}"));
                add_to_point_map(&mut point_map, (x_start, parsed_line.len()), y, value);
                num.clear();

                None
            });
        }

        parsed_schematic.push(parsed_line);
    }

    let mut part_number_sum = 0;
    let mut gear_ratio_sum = 0;
    for (y, line) in parsed_schematic.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            // PART 1
            if !point.is_ascii_digit() && *point != '.' {
                let adj_nums =
                    find_all_nums_adj((x, y), (line.len(), parsed_schematic.len()), &mut point_map);
                part_number_sum += adj_nums.iter().sum::<u32>();
            }
            // PART 2
            if *point == '*' {
                let adj_nums =
                    find_all_nums_adj((x, y), (line.len(), parsed_schematic.len()), &mut point_map);
                if adj_nums.len() == 2 {
                    gear_ratio_sum += adj_nums.iter().product::<u32>()
                }
            }
        }
    }

    (part_number_sum, gear_ratio_sum)
}

fn find_all_nums_adj(
    point: Point,
    bounds: Range,
    point_map: &mut HashMap<Point, PartNumber>,
) -> Vec<u32> {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let adj_points = adjacent_points(point, bounds);
    for point in adj_points {
        if let Some(part_number) = point_map.get(&point) {
            if is_new_part(&part_numbers, &part_number) {
                part_numbers.push(part_number.clone());
            }
        }
    }

    part_numbers.iter().map(|part| *part.borrow()).collect()
}

fn is_new_part(parts_seen: &Vec<PartNumber>, part: &PartNumber) -> bool {
    for p in parts_seen {
        if Rc::ptr_eq(&p, &part) {
            return false;
        }
    }

    true
}

fn adjacent_points(p: Point, size: Range) -> Vec<Point> {
    let (p_x, p_y) = p;

    let mut adj = vec![
        // left col
        // (p_x - 1, p_y - 1),
        // (p_x - 1, p_y),
        // (p_x - 1, p_y + 1),
        // top
        // (p_x, p_y - 1),
        // (p_x + 1, p_y - 1),
        // right
        (p_x + 1, p_y),
        (p_x + 1, p_y + 1),
        // bottom
        (p_x, p_y + 1),
    ];
    if p_x > 0 && p_y > 0 {
        adj.extend(vec![
            (p_x - 1, p_y - 1),
            (p_x - 1, p_y),
            (p_x - 1, p_y + 1),
            (p_x, p_y - 1),
            (p_x + 1, p_y - 1),
        ]);
    } else if p_x > 0 {
        adj.extend(vec![(p_x - 1, p_y), (p_x - 1, p_y + 1)]);
    } else if p_y > 0 {
        adj.extend(vec![(p_x + 1, p_y - 1), (p_x, p_y - 1)]);
    }

    adj.iter()
        .filter(|(x, y)| *x < size.0 && *y < size.1)
        .map(|(x, y)| (*x, *y))
        .collect()
}

fn add_to_point_map(map: &mut HashMap<Point, PartNumber>, range: Range, y: usize, value: u32) {
    let value = Rc::new(RefCell::new(value));
    for x in range.0..range.1 {
        map.insert((x, y), value.clone());
    }
}

#[cfg(test)]
mod test {
    use crate::sum_engine_schematic;

    #[test]
    fn day3_simple_schematic() {
        let test_schematic = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let sums = sum_engine_schematic(test_schematic);
        assert!(sums.0 == 4361, "part sum is {}", sums.0);
        assert!(sums.1 == 467835, "gear ratio is {}", sums.1);
    }
}
