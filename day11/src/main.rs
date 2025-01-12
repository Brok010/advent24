use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let stones = parsing(input);
    let part1 = solve(&stones, 25);
    let part2 = solve(&stones, 75);

    println!("{}, {}", part1, part2);
}

fn solve(stones: &Vec<i64>, ticks: i32) -> i32 {
    let mut result = 0;
    let mut cache: HashMap<(i64, i32), i32> = HashMap::new(); // stone, ticks -> result
    
    for &stone in stones {
        if let Some(&value) = cache.get(&(stone, ticks)) {
            result += value;
        } else {
            let new_result = get_result(stone, 0, ticks, &mut cache);
            result += new_result;
        }
    }

    result
}

fn get_result(stone: i64, current_ticks: i32, ticks: i32, cache: &mut HashMap<(i64, i32), i32>) -> i32 {
    if current_ticks == ticks {
        return 1;
    }

    let new_cache_entry = (stone, current_ticks);
    if let Some(&value) = cache.get(&new_cache_entry) {
        return value;
    }

    let new_stones = get_new_stones(stone);
    let mut result = 0;

    for ns in new_stones {
        result += get_result(ns, current_ticks + 1, ticks, cache);
    }

    cache.insert(new_cache_entry, result); // Cache the computed result
    result
}

fn get_new_stones(stone: i64) -> Vec<i64> {
    if stone == 0 {
        vec![1]
    } else if let Some(splitted) = split_even_digits(stone) {
        splitted
    } else {
        vec![stone * 2024]
    }
}

fn split_even_digits(num: i64) -> Option<Vec<i64>> { //totaly not chatgpt
    let num_str = num.to_string(); // Convert the number to a string
    let len = num_str.len();

    if len % 2 != 0 {
        return None; // Return None if the number of digits is odd
    }

    let mid = len / 2;
    let first_half = num_str[..mid].parse::<i64>().unwrap(); // Parse the first half
    let second_half = num_str[mid..].parse::<i64>().unwrap(); // Parse the second half

    Some(vec![first_half, second_half]) // Return the halves as a vector
}


fn parsing(input: &str) -> Vec<i64> {
    input
        .split_whitespace() 
        .filter_map(|s| s.parse::<i64>().ok()) 
        .collect()
}

