use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day4, part1)]
fn part1(input: &str) -> i32 {
    let grid: Vec<String> = input.lines().map(|v| v.to_string()).collect();
    
    let mut total = 0;
    total += horizontal(grid.clone());
    total += vertical(grid.clone());
    total += diagonal(grid.clone());

    return total;
}

fn print_puzzle(puzzle: Vec<String>) {
    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            print!("{}", puzzle[y].as_bytes()[x] as char);
        }
        println!();
    }
    println!();
}

fn find_xmas(lines: Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines {
        sum += line.matches("XMAS").collect::<Vec<_>>().len();
        sum += line.matches("SAMX").collect::<Vec<_>>().len();
    }
    return sum as i32;
}

fn horizontal(puzzle: Vec<String>) -> i32 {
    return find_xmas(puzzle);
}

fn vertical(puzzle: Vec<String>) -> i32 {
    let mut chars = vec![vec!['.'; puzzle.len()]; puzzle[0].len()];
    for x in 0..puzzle[0].len() {
        for y in 0..puzzle.len() {
            chars[x][y] = puzzle[y].as_bytes()[x] as char;
        }
    }

    let mut lines = vec!("...".to_string(); chars.len());
    for i in 0..chars.len() {
        lines[i] = chars[i].iter().collect::<String>();
    }

    return find_xmas(lines);
}

fn diagonal(puzzle: Vec<String>) -> i32 {
    let mut d1: Vec<String> = vec!("".to_string(); puzzle.len() + puzzle[0].len() - 1);
    let mut d2: Vec<String> = vec!("".to_string(); puzzle.len() + puzzle[0].len());
    
    for x in 0..puzzle[0].len() {
        for y in 0..puzzle.len() {
            d1[x+y].push(puzzle[y].as_bytes()[x] as char);
        }
    }

    for i in 0..puzzle.len() + puzzle[0].len() {
        let mut s = "".to_string();
        let mut x;
        let mut y;
        if i == puzzle[0].len() {
            continue;
        }
        if i < puzzle[0].len() {
            x = i;
            y = 0;
        }
        else {
            x = 0;
            y = i-puzzle[0].len();
        }

        while x < puzzle[0].len() && y < puzzle.len(){
            s.push(puzzle[y].as_bytes()[x] as char);
            x += 1;
            y += 1;
        }
        d2[i] = s;
    }

    return find_xmas(d1) + find_xmas(d2);
}

#[aoc(day4, part2)]
fn part2(input: &str) -> i32 {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX")), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 0);
    }
}
