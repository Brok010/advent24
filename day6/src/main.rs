// obstruction can't be placed at the guard's starting position
static DIRECTION: [[i32; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

fn main() {
    let input = include_str!("input.txt");
    let map = parsing(input);

    let (start_x, start_y) = get_starting_pos(&map);
    let start_direction = 0;

    let parts = count_parts(start_x, start_y, start_direction, &map);
    println!("{}",parts.0);
    println!("{}",parts.1); //1846 and 7 too high
}

fn try_to_make_loop(start_x: i32, start_y: i32, start_direction: i32, mut map: Vec<Vec<char>>) -> i32 {
    let mut current_x = start_x;
    let mut current_y = start_y;
    let mut loop_steps: Vec<(i32, i32, i32)> = Vec::new();
    //update direction to right - simulates the obstacle
    let mut new_dir = get_next_dir(start_direction);

    // add # to the maps
    let new_x = (start_x + DIRECTION[start_direction as usize][0]) as usize;
    let new_y = (start_y + DIRECTION[start_direction as usize][1]) as usize;
    if new_x < map[0].len() && new_y < map.len() {
        map[new_y][new_x] = '#';
    }


    loop_steps.push((start_x, start_y, start_direction));
    
    //simulate the rest of walk
    loop {
        let (next_x, next_y) = get_next_pos(&DIRECTION[new_dir as usize], current_x, current_y);
        // println!("{}, {}", next_x, next_y);
        
        // if its out of bounds
        if next_x < 0 || next_x >= map[0].len() as i32 || next_y < 0 || next_y >= map.len() as i32 {
            return 0;
        }
        
        if map[next_y as usize][next_x as usize] == '#' { // if we cant make a step change dir
            new_dir = get_next_dir(new_dir);
        } else {
            // check if the next step isnt the one we simulated at the begining else make the step
            // println!("if {} == {} && {} == {} && {} == {}", next_x, start_x, next_y, start_y, new_dir, start_direction);
            if next_x == start_x && next_y == start_y && new_dir == start_direction {
                return 1;
            } else {
                current_x = next_x;
                current_y = next_y;
                let new_step_entry = (current_x, current_y, new_dir);
                if loop_steps.contains(&new_step_entry) {
                    return 1;
                } else {
                    loop_steps.push(new_step_entry);
                }    
            }
        }
        // println!("{}", map[next_x as usize][next_y as usize])
    }
}

fn count_parts(start_x: i32, start_y: i32, start_direction: i32, map: &Vec<Vec<char>>) -> (i32, i32) {
    let mut steped_into: Vec<(i32, i32)> = Vec::new();
    let mut current_x = start_x;
    let mut current_y = start_y;
    let mut current_direction = start_direction;
    let mut part2 = 0; 
    let mut tried_for_tiles: Vec<(i32, i32)> = Vec::new(); //count tiles only once

    loop {
        let (next_x, next_y) = get_next_pos(&DIRECTION[current_direction as usize], current_x, current_y);
        // println!("{}, {}", next_x, next_y);
        
        // if its out of bounds
        if next_x < 0 || next_x >= map[0].len() as i32 || next_y < 0 || next_y >= map.len() as i32 {
            break;
        }
        
        if map[next_y as usize][next_x as usize] == '#' { // if we cant make a step change dir
            current_direction = get_next_dir(current_direction);
        } else { // make a step
            // println!("{}, {}", next_x, next_y);
            // add this for part 1
            if !steped_into.contains(&(current_x, current_y)) {
                steped_into.push((current_x, current_y));
            }
            // part 2 - if there is not an obstacle we try to put one in there and see if it loops
            if !tried_for_tiles.contains(&(next_x, next_y)) {
                let new_loop_entry = (current_x, current_y, current_direction);
                let loop_posibility = try_to_make_loop(new_loop_entry.0, new_loop_entry.1, new_loop_entry.2, map.clone());
                if loop_posibility == 1 {
                    tried_for_tiles.push((next_x, next_y));
                }
                part2 += loop_posibility;
            }
                       
            current_x = next_x;
            current_y = next_y;
        }
        // println!("{}", map[next_x as usize][next_y as usize])
    }
    return (steped_into.len() as i32, part2);
}

fn get_next_dir(start_dir: i32) -> i32 {
    let mut current_direction: i32;
    
    if start_dir < 3 {
        current_direction = start_dir + 1;
    } else {
        current_direction = 0;
    }
    
    current_direction
}

fn get_next_pos(dir: &[i32; 2], x: i32, y: i32) -> (i32, i32) {
    let new_x = x + dir[0];
    let new_y = y + dir[1];
    (new_x, new_y)
}

fn get_starting_pos(map: &Vec<Vec<char>>) -> (i32, i32) {
    let start_x = 0;
    let start_y = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '^' {
                return (x as i32, y as i32);
            }
        }
    }

    (start_x, start_y)
}

fn parsing(input: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        map.push(chars);
    }
    map
}
