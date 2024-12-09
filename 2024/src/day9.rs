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
fn part2(input: &str) -> u64 {
    let mut disk: Vec<Vec<String>> = vec!();
    
    let mut index = 0;
    let mut id = 0;
    for size in input.chars().map(|c| c.to_digit(10).unwrap()) {
        let mut s = ".".to_string();
        if index % 2 == 0 {
            s = id.to_string();
            id += 1;
        }
        let mut v: Vec<String> = vec!();
        for _ in 0..size {
            v.push(s.clone());
        }
        if v.len() != 0 {
            disk.push(v);
        }
        index += 1;
    }

    // println!("{}", disk.clone().into_iter().flatten().collect::<Vec<String>>().join(""));

    let mut i = (disk.len() - 1) as i32;
    while i > 0 {
        let mut j = 0;
        while disk[i as usize][0] == ".".to_string() {
            i -= 1;
        }
        while !(disk[j][0] == ".".to_string() && disk[j].len() >= disk[i as usize].len()) {
            j += 1;
            if j >= disk.len() {
                break;
            }
        }
        if j > i as usize {
            i -= 1;
            continue;
        }
        //println!("i: {:?}, j:{:?}", disk[i as usize], disk[j]);
        if disk[j].len() == disk[i as usize].len() {
            disk[j] = disk[i as usize].clone();
        disk[i as usize] = vec!(".".to_string(); disk[i as usize].len()); 
        }
        else {
            let remainder: i32 = disk[j].len() as i32 - disk[i as usize].len() as i32;
            if remainder < 0 {
                println!("something is horribly wrong");
                break;
            }
            disk[j] = disk[i as usize].clone();
            disk[i as usize] = vec!(".".to_string(); disk[i as usize].len()); 
            disk.insert(j + 1, vec!(".".to_string(); remainder as usize));
        }
    }

    //println!("{}", disk.clone().into_iter().flatten().collect::<Vec<String>>().join(""));

    let mut checksum = 0;
    let mut i = 0;
    for f in disk.clone()
        .into_iter()
        .flatten()
        .collect::<Vec<String>>() {
        if !f.contains(".") {
            checksum += f.parse::<u64>().unwrap() * i;
        }
        i += 1;
    }

    return checksum;
}
