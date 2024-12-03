fn is_safe(report: Vec<i32>) -> bool {
    let mut s = report[0] - report[1];
    if s == 0 {
        return false;
    }
    s = s / s.abs();

    for i in 1..report.len() - 1 {
        if report[i] - report[i + 1] == 0 {
            return false;
        }
        if s != (report[i] - report[i + 1]) / (report[i] - report[i + 1]).abs() {
            return false;
        }
    }

    for i in 0..report.len() - 1 {
        if (report[i] - report[i + 1]).abs() > 3 {
            return false;
        }
    }
    return true;
}

#[aoc(day2, part1)]
pub fn part_one(input: &str) -> i32 {
    let input_lines = input.lines();

    let mut reports =
        Vec::<Vec<i32>>::with_capacity(input_lines.clone().collect::<Vec<&str>>().len());

    let mut i = 0;
    for line in input_lines {
        reports.insert(i, vec![]);
        let levels = line.split(" ").collect::<Vec<&str>>();
        for level in levels {
            reports[i].push(level.parse::<i32>().unwrap());
        }
        i += 1;
    }

    let mut safe_reports = 0;
    for report in reports.clone() {
        if is_safe(report) {
            safe_reports += 1;
        }
    }
    return safe_reports;
}

#[aoc(day2, part2)]
pub fn part_two(input: &str) -> i32 {
    let input_lines = input.lines();
    let mut reports =
        Vec::<Vec<i32>>::with_capacity(input_lines.clone().collect::<Vec<&str>>().len());

    let mut i = 0;
    for line in input_lines {
        reports.insert(i, vec![]);
        let levels = line.split(" ").collect::<Vec<&str>>();
        for level in levels {
            reports[i].push(level.parse::<i32>().unwrap());
        }
        i += 1;
    }

    let mut safe_reports = 0;
    for report in reports.clone() {
        if is_safe(report.clone()) {
            safe_reports += 1;
            continue;
        }
        for i in 0..report.len() {
            let mut modified_report = report.clone();
            _ = modified_report.remove(i);
            if is_safe(modified_report) {
                safe_reports += 1;
                break;
            }
        }
    }
    return safe_reports;
}
