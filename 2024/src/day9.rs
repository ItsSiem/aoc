use aoc_runner_derive::{aoc, aoc_generator};
#[aoc(day9, part1)]
fn part1(input: &str) -> u64 {
    let mut disk: Vec<String> = vec!();
    
    let mut index = 0;
    let mut id = 0;
    for size in input.chars().map(|c| c.to_digit(10).unwrap()) {
        let mut s = ".".to_string();
        if index % 2 == 0 {
            s = id.to_string();
            id += 1;
        }
        for _ in 0..size {
            disk.push(s.clone());
        }
        index += 1;
    }

    //println!("{}", disk.join(""));

    let mut i = disk.len() - 1;
    let mut j = 0; 
    while i > j {
        while disk[j] != ".".to_string() {
            j += 1;
        }
        while disk[i] == ".".to_string() {
            i -= 1;
        }
        if i < j {
            break;
        }
        disk[j] = disk[i].clone();
        disk[i] = ".".to_string();
    }

    //println!("{}", disk.join(""));

    let mut checksum = 0_u64;
    let mut i = 0_u64;
    for f in disk.into_iter()
        .filter(|s| s != ".")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>() {
        checksum += i * f as u64;
        i += 1;
    }

    return checksum;
}

#[aoc(day9, part2)]
fn part2(input: &str) -> String {
    todo!()
}
