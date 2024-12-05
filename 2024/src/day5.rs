use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {
    let split_input = input.split("\n\n").collect::<Vec<&str>>();
    let rules = get_rules(split_input[0]);
    let updates = get_updates(split_input[1]);
    
    let mut total = 0;
    for update in updates {
        if is_update_valid(update.clone(), rules.clone()) {
            let mid = update.len() / 2;
            total += update[mid];
        }
    }
    return total;
}

fn get_rules(rules_string: &str) -> HashMap<i32, Vec<i32>> {
    let mut rules = HashMap::new();
    for rule_string in rules_string.lines() {
        let split_rule = rule_string.split("|").collect::<Vec<&str>>();
        let page = split_rule[1].parse::<i32>().expect("couldnt parse page");
        let condition = split_rule[0].parse::<i32>().expect("couldnt parse conditon");
        if !rules.contains_key(&page) {
            let conditions = vec!(condition);
            rules.insert(page, conditions);
        }
        else {
            rules.get_mut(&page).unwrap().push(condition);
        }
    }
    return rules;
}

fn get_updates(updates_string: &str) -> Vec<Vec<i32>> {
    let mut updates = Vec::new();
    for update_string in updates_string.lines() {
        let mut update = Vec::new();
        for page in update_string.split(",").collect::<Vec<&str>>().iter() {
            update.push(page.parse::<i32>().unwrap());
        }
        updates.push(update);
    }

    return updates;
}

fn is_update_valid(update: Vec<i32>, rules: HashMap<i32, Vec<i32>>) -> bool {
    let mut visited = HashSet::new();
    for page in update.iter() {
        if rules.get(&page).is_none() {
            visited.insert(page);
            continue;
        }

        for condition in rules.get(&page).unwrap() {
            if !visited.contains(condition) && update.contains(condition) {
                return false;
            }
        }
        visited.insert(page);
    }
    return true;
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    todo!()
}
