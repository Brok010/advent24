use std::str::FromStr;
struct Equation {
    result: i64,
    parts: Vec<i64>
}

fn main() {
    let input = include_str!("input.txt");
    let equations = parsing(input);
    let parts = parts(equations);
    println!("{:?}", parts);
}

fn parts(equations: Vec<Equation>) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;
    let oper_part2: &[char] = &['+', '*', '|'];
    let oper_part1: &[char] = &['+', '*'];

    for e in equations {
        part1 += can_be_solved(&e, oper_part1);
        part2 += can_be_solved(&e, oper_part2)
    }
    (part1, part2)
}

fn can_be_solved(equation: &Equation, oper: &[char]) -> i64 {
    let operators = equation.parts.len() - 1; // Number of operators needed
    let choices = generate_combinations(operators, oper);
    for choice in choices {
        let mut result = 0;
        result = equation.parts[0];
        for i in 0..operators {
            match choice[i] {
                '*' => result *= equation.parts[i + 1],
                '+' => result += equation.parts[i + 1],
                '|' => result = concate_result(result, equation.parts[i + 1]),
                _ => println!("U fcked up")
            }
        }
        if result == equation.result {
            return result;
        }
    }
    return 0;
}

fn concate_result(result: i64, next_num: i64) -> i64 {
    let concatenated = format!("{}{}", result, next_num);
    concatenated.parse::<i64>().unwrap_or(0)
}

fn generate_combinations(n: usize, options: &[char]) -> Vec<Vec<char>> {
    let mut results = Vec::new();
    let mut current = Vec::new();

    fn helper(
        n: usize,
        options: &[char],
        current: &mut Vec<char>,
        results: &mut Vec<Vec<char>>,
    ) {
        if current.len() == n {
            results.push(current.clone());
            return;
        }

        for &option in options {
            current.push(option);
            helper(n, options, current, results);
            current.pop(); // Backtrack
        }
    }

    helper(n, options, &mut current, &mut results);
    results
}


fn parsing(input: &str) -> Vec<Equation> {
    let mut equations: Vec<Equation> = Vec::new();

    for line in input.lines() {
        let line_cleaned = line.replace(":", "");

        let mut numbers = line_cleaned.split_whitespace()
        .filter_map(|part| i64::from_str(part).ok());
    
        if let Some(result) = numbers.next() {
            let parts: Vec<i64> = numbers.collect();
            equations.push(Equation { result, parts });
        }
    }
    equations
}
