#[aoc(day1, part1)]
pub fn part_one(input: &str) -> i32 {
    let mut a = Vec::<i32>::new();
    let mut b = Vec::<i32>::new();
    let lines = input.lines();
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
    return sum;
}

#[aoc(day1, part2)]
pub fn part_two(input: &str) -> i32 {
    let mut a = Vec::<i32>::new();
    let mut b = Vec::<i32>::new();
    let lines = input.lines();
    for line in lines {
        let entries = line.split("   ").collect::<Vec<&str>>();

        a.push(entries[0].parse::<i32>().unwrap());
        b.push(entries[1].parse::<i32>().unwrap());
    }
    a.sort();
    b.sort();

    let mut sum = 0;
    for e in a {
        let mut c = 0;
        for d in &b {
            if e == *d {
                c += 1;
            }
        }
        sum += e * c;
    }
    return sum;
}
