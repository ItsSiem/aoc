use regex::Regex;

#[aoc(day3, part1)]
pub fn part_one(input: &str) -> i32 {
    let mut sum = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for cap in re.captures_iter(&input) {
        sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
    }
    return sum;
}

#[aoc(day3, part2)]
pub fn part_two(input: &str) -> i32 {
    let mut sum = 0;
    let mut enabled = true;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    for cap in re.captures_iter(&input) {
        if cap.get(3).is_some() {
            enabled = true;
            continue;
        }
        if cap.get(4).is_some() {
            enabled = false;
            continue;
        }
        if !enabled {
            continue;
        }
        sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
    }
    return sum;
}
