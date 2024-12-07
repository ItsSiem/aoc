use aoc_runner_derive::{aoc, aoc_generator};
#[aoc(day7, part1)]
fn part1(input: &str) -> u64 {
    let eqs = input.lines().collect::<Vec<&str>>()
        .into_iter().map(|s| parse_equation(s.to_string()))
        .collect::<Vec<Equation>>();

    println!("{:?}", eqs.len());
    let mut total = 0;
    for e in eqs {
        if generate_possible_equations(e.clone()).iter().filter(|ee| evaluate_equation(ee.clone().clone())).count() > 0 {
                total += e.result;
        }
    }

    return total as u64;
}

#[derive(Clone)]
#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
    operators: Vec<Operator>,
}

#[derive(Clone)]
#[derive(Debug)]
enum Operator {
    Addition,
    Multiplication,
}

fn generate_possible_equations(eq: Equation) -> Vec<Equation> {
    let operator_count = eq.operands.len() - 1;
    
    fn generate_operator_permutations(len: usize) -> Vec<Vec<Operator>> {
        if len == 0 {
            return vec![vec![]];
        }

        let mut results = vec![];
        for mut perm in generate_operator_permutations(len - 1) {
            let mut perm_add = perm.clone();
            perm.push(Operator::Addition);
            perm_add.push(Operator::Multiplication);
            results.push(perm);
            results.push(perm_add);
        }
        return results;
    }

    let operator_permutations = generate_operator_permutations(operator_count);

    let mut equations = vec![];
    for operator_sequence in operator_permutations {
        equations.push(Equation {
            result: eq.result.clone(),
            operands: eq.operands.clone(),
            operators: operator_sequence,
        });
    }

    return equations;
}

fn parse_equation(s: String) -> Equation {
    let mut eq = Equation {
        result: 0,
        operands: vec!(),
        operators: vec!(),
    };
    let elements = s.split(" ").collect::<Vec<&str>>();

    eq.result = elements[0][..elements[0].len() - 1]
        .parse::<u64>().expect("Couldn't parse eqation result");
    for element in &elements[1..] {
        eq.operands.push(element.parse::<u64>().expect("Couldn't parse element of equation"));
    }

    return eq;
}

fn evaluate_equation(eq: Equation) -> bool {
    let mut total = eq.operands[0];
    for i in 0..eq.operators.len() {
        let b = eq.operands[i+1];
        match eq.operators[i] {
            Operator::Addition => {
                total += b;
            },
            Operator::Multiplication => {
                total *= b;
            },
        }
    }
    return total == eq.result;
}

#[aoc(day7, part2)]
fn part2(input: &str) -> i32 {
    todo!()
}
