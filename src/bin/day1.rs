use aoc::Part;

fn main() {
    let (problem, mut context) = aoc::setup_day();

    let mut allow_list = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    if problem.part == Part::P2 {
        allow_list.append(&mut vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]);
    }

    let mut calibration = 0;
    while let Some(line) = context.get_line() {
        let (left, right) = find_num_pairs(&line, &allow_list);
        calibration += left * 10 + right;
    }
    println!("calibration value: {calibration}");
}

fn find_num_pairs(line: &str, allow_list: &[&str]) -> (u32, u32) {
    // use find and rfind
    let mut lowest = None;
    let mut left_num = 0;
    for num in allow_list {
        let index = match line.find(num) {
            Some(x) => x,
            None => continue,
        };

        if let Some(low_index) = lowest {
            if index < low_index {
                lowest = Some(index);
                left_num = translate_num_to_int(num);
            }
        } else {
            lowest = Some(index);
            left_num = translate_num_to_int(num);
        }
    }

    let mut highest = None;
    let mut right_num = 0;
    for num in allow_list {
        let index = match line.rfind(num) {
            Some(x) => x,
            None => continue,
        };

        if let Some(high_index) = highest {
            if index > high_index {
                highest = Some(index);
                right_num = translate_num_to_int(num);
            }
        } else {
            highest = Some(index);
            right_num = translate_num_to_int(num);
        }
    }

    (left_num, right_num)
}

fn translate_num_to_int(num: &str) -> u32 {
    match num {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("don't do that"),
    }
}
