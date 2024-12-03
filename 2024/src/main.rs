use regex::Regex;
use std::fs;

fn main() {
    day_one();
    day_two();
    day_three();
}

fn day_three() {
    println!("== Day Three ==");

    let file_path = "inputs/03";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut sum = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for cap in re.captures_iter(&input) {
        sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
    }
    println!("Part one: {}", sum);
}

fn day_two() {
    println!("== Day Two ==");
    let file_path = "inputs/02";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
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
    println!("Part one: {}", safe_reports);

    safe_reports = 0;
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
    println!("Part two: {}", safe_reports);
}

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

fn day_one() {
    println!("== Day One ==");
    let file_path = "inputs/01";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut a = Vec::<i32>::new();
    let mut b = Vec::<i32>::new();
    let lines = contents.lines();
    for line in lines {
        let entries = line.split("   ").collect::<Vec<&str>>();

        a.push(entries[0].parse::<i32>().unwrap());
        b.push(entries[1].parse::<i32>().unwrap());
    }
    a.sort();
    b.sort();

    let mut sum = 0;
    let mut i = 0;
    while i < a.len() {
        sum += (a[i] - b[i]).abs();
        i += 1;
    }
    println!("Part one: {}", sum);

    sum = 0;
    for e in a {
        let mut c = 0;
        for d in &b {
            if e == *d {
                c += 1;
            }
        }
        sum += e * c;
    }

    println!("Part two: {}", sum);
}
