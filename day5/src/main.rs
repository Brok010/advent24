use std::process::Output;


#[derive(Debug)]
struct RuleBook {
    sequence: Vec<i32>,
    rules: Vec<(i32, i32)>
}

fn main() {
    let input = include_str!("input.txt");
    let (rules, sequences) = parsing(input);

    // for rule in rules {
    //     println!("{:?}", rule);
    // }
    // println!("{:?}", sequences);

    let parts = count_part1(&rules, &sequences);
    println!("{:?}", parts);
    // let part2 = count_part2(&rules, &sequences);
}

fn count_part1(rules: &Vec<(i32, i32)>, sequences: &Vec<Vec<i32>>) -> (i32, i32) {
    let mut rulebooks: Vec<RuleBook> = Vec::new();

    for sequence in sequences {
        let mut relevant_rules: Vec<(i32, i32)> = Vec::new();
        for rule in rules {
            if sequence.contains(&rule.0) && sequence.contains(&rule.1) {
                relevant_rules.push(*rule);
            }
        }
        let rule_book = RuleBook {
            sequence: sequence.clone(),
            rules: relevant_rules,
        };

        rulebooks.push(rule_book);
        
    }

    let counter = process_rulebook(rulebooks);

    counter
}

fn process_rulebook(rulebooks: Vec<RuleBook>) -> (i32, i32) {
    let mut part1 = 0;
    let mut part2 = 0;

    for each in rulebooks {
        let mut valid = true;
        // println!("{:?}", each);
        for rule in &each.rules {
            let first_rule_num = rule.0;
            let second_rule_num = rule.1;
            for (i, &num) in each.sequence.iter().enumerate() {
                if num == first_rule_num {
                    for (j, &num2) in each.sequence.iter().enumerate() {
                        if num2 == second_rule_num && i > j {
                            // println!("{:?}", each.sequence);
                            // println!("{} | {}, ({}, {}), ({}, {})", first_rule_num, second_rule_num, each.sequence[i], i, each.sequence[j], j);
                            part2 += repair_sequence(&each.sequence, &each.rules);
                            valid = false;
                            break;
                        }
                    }
                }
                if valid == false {         
                    break;
                }
            }
            if valid == false {
                break;
            }  
        }      
        if valid != false {
            part1 += each.sequence[(each.sequence.len() - 1) / 2 ]; //asuming all sequences have length of odd num
        }
    }

    (part1, part2)
}

fn repair_sequence(sequence: &Vec<i32>, rules: &Vec<(i32, i32)>) -> i32 {
    let mut output = 0;
    let mut new_sequence = sequence.clone();

    loop {
        let mut repaired = false;
        for rule in rules {
            let first_rule_num = rule.0;
            let second_rule_num = rule.1;
            for  i in 0..sequence.len() {
                if sequence[i] == first_rule_num {
                    for j in 0..sequence.len() {
                        if sequence[j] == second_rule_num && i > j{
                            let temp = new_sequence[i];
                            new_sequence[i] = new_sequence[j];
                            new_sequence[j] = temp;
                            repaired = true;
                        }
                    }
                }
            }
        }
    }

    output = repaired_sequence[(repaired_sequence.len() - 1) / 2];
}

fn parsing(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut sequences: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        if line.contains('|') {
            let rule_parts: Vec<i32> = line
                .split('|') // Split the line into parts by '|'
                .map(|x| x.trim()) // Trim whitespace around each part
                .filter_map(|x| x.parse::<i32>().ok()) // Parse each part into an i32, ignore invalid parts
                .collect();
        
            if rule_parts.len() == 2 {
                rules.push((rule_parts[0], rule_parts[1]));
            } else {
                eprintln!("Invalid rule format: {}", line);
            }
        } else if !line.is_empty() {
            let sequence: Vec<i32> = line
                .split(',')
                .filter_map(|x| x.trim().parse::<i32>().ok())
                .collect();
            sequences.push(sequence);
        }
    }
    (rules, sequences)
}
