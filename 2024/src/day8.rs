use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[aoc(day8, part1)]
fn part1(input: &str) -> i32 {
    let mut charmap = input.lines().collect::<Vec<&str>>()
        .into_iter().map(|s| {s.chars().collect::<Vec<char>>()})
        .collect::<Vec<Vec<char>>>();

    let mut antennas: HashMap<char, Vec<Position>> = charmap.clone()
        .into_iter()
        .flatten()
        .filter(|c| *c != '.')
        .map(|c| (c, vec!()))
        .collect();

    for y in 0..charmap.len() {
        for x in 0..charmap[0].len() {
            if antennas.contains_key(&charmap[y][x]) {
                antennas.get_mut(&charmap[y][x])
                    .expect("No antenna found")
                    .push(Position { x: x as i32, y: y as i32});
            }
        }
    }

    let mut antinodes = 0;
    let mut antinode_positions: HashSet<Position> = HashSet::new();
    for freq in antennas.keys() {
        for a in &antennas[freq] {
            for b in &antennas[freq] {
                let distance_sq = (a.x - b.x).pow(2) * (a.y - b.y).pow(2);
                if distance_sq == 0 {
                    continue;
                }
                let distance_x = a.x - b.x;
                let distance_y = a.y - b.y;
                let antinode_x = a.x + distance_x;
                let antinode_y = a.y + distance_y;
                if antinode_x < charmap[0].len() as i32 && 
                    antinode_x >= 0 &&
                    antinode_y < charmap.len() as i32 &&
                    antinode_y >= 0 &&
                    !antinode_positions.contains(&Position {x: antinode_x, y: antinode_y}) {
                    antinodes += 1;
                    // charmap[antinode_y as usize][antinode_x as usize] = '#';
                    antinode_positions.insert(Position {x: antinode_x, y: antinode_y});
                }
            }
        }
    }
    // draw_charmap(charmap);

    return antinodes;
}

fn draw_charmap(charmap: Vec<Vec<char>>) {
    for l in charmap {
        for c in l {
            print!("{}", c);
        }
        println!();
    }
}

#[aoc(day8, part2)]
fn part2(input: &str) -> String {
    todo!()
}
