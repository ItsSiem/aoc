use aoc_runner_derive::{aoc, aoc_generator};
#[aoc(day6, part1)]
fn part1(input: &str) -> i32 {
    let mut map = input.to_string();
    println!("{}\n", map);

    while map.contains("^") | map.contains(">") | map.contains("v") | map.contains("<") {
        map = step(map);
    }

    println!("{}\n", map);
    return map.chars().filter(|c| *c == 'X').count() as i32;
}

fn step(map: String) -> String {
    let mut charmap = map.lines().collect::<Vec<&str>>()
                    .into_iter().map(|s| {s.chars().collect::<Vec<char>>()})
                    .collect::<Vec<Vec<char>>>();
    let mut direction = '^';
    let mut currentpos = (0, 0);
    for x in 0..charmap[0].len() {
        for y in 0..charmap.len() {
            match charmap[y][x] {
                '^' => {
                    direction = '^';
                    currentpos = (x, y);
                },
                '>' => {
                    direction = '>';
                    currentpos = (x, y);
                },
                'v' => {
                    direction = 'v';
                    currentpos = (x, y);
                },
                '<' => {
                    direction = '<';
                    currentpos = (x, y);
                },
                _ => {
                    ();
                }
            }
        }
    }
    
    let mut nextpos = match direction {
        '^' => (currentpos.0, currentpos.1 - 1),
        '>' => (currentpos.0 + 1, currentpos.1),
        'v' => (currentpos.0, currentpos.1 + 1),
        '<' => (currentpos.0 - 1, currentpos.1),
        _ => panic!(),
    };

    if nextpos.0 >= charmap[0].len() || 
        nextpos.1 >= charmap.len() {
        charmap[currentpos.1][currentpos.0] = 'X'; 
        return charmap.iter().map(|l| l.iter().collect::<String>() + "\n").collect();
    }

    if charmap[nextpos.1][nextpos.0] == '#'  {
        direction = match direction {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => panic!(),
        };
        nextpos = match direction {
            '^' => (currentpos.0, currentpos.1 - 1),
            '>' => (currentpos.0 + 1, currentpos.1),
            'v' => (currentpos.0, currentpos.1 + 1),
            '<' => (currentpos.0 - 1, currentpos.1),
            _ => panic!(),
        };
    }

    let chartrail = match direction {
            '^' => 'n',
            '>' => 'e',
            'v' => 's',
            '<' => 'w',
            _ => panic!(),
    };
        

    charmap[nextpos.1][nextpos.0] = direction; 
    charmap[currentpos.1][currentpos.0] = chartrail; 

    return charmap.iter().map(|l| l.iter().collect::<String>() + "\n").collect();
}

#[aoc(day6, part2)]
fn part2(input: &str) -> i32 {
    todo!()
}
