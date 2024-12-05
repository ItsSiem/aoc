use std::{cmp::Ordering, collections::{HashMap, HashSet}};

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

fn get_rules(rules_string: &str) -> HashMap<i32, HashSet<i32>> {
    let mut rules = HashMap::new();
    for rule_string in rules_string.lines() {
        let split_rule = rule_string.split("|").collect::<Vec<&str>>();
        let page = split_rule[1].parse::<i32>().expect("couldnt parse page");
        let condition = split_rule[0].parse::<i32>().expect("couldnt parse conditon");
        if !rules.contains_key(&page) {
            let mut conditions: HashSet<i32> = HashSet::new();
            conditions.insert(condition);
            rules.insert(page, conditions);
        }
        else {
            rules.get_mut(&page).unwrap().insert(condition);
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

fn is_update_valid(update: Vec<i32>, rules: HashMap<i32, HashSet<i32>>) -> bool {
    let mut visited = HashSet::new();
    for page in update.iter() {
        if !rules.contains_key(&page) {
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
fn part2(input: &str) -> i32 {
    let split_input = input.split("\n\n").collect::<Vec<&str>>();
    let rules = get_rules(split_input[0]);
    let updates = get_updates(split_input[1]);

    let mut incorrect_updates = updates.iter()
        .filter(|u| !is_update_valid(u.to_vec(), rules.clone()))
        .cloned()
        .collect::<Vec<Vec<i32>>>();

    let mut total = 0;
    for update in incorrect_updates.iter_mut() {
        update.sort_by(|lhs, rhs| compare(*lhs, *rhs, rules.clone()));
        total += update[update.len()/2];
    }
    return total;
}

fn compare(lhs: i32, rhs: i32, rules: HashMap<i32, HashSet<i32>>) -> Ordering {
    if rules.contains_key(&rhs) && rules[&rhs].contains(&lhs) {
        return Ordering::Less;
    }
    if rules.contains_key(&lhs) && rules[&lhs].contains(&rhs) {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}
