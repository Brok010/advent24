
static DIRECTIONS: [[i32; 2]; 4] = [[1, 0], [0, -1], [-1, 0], [0, 1]];
fn main() {
    let input = include_str!("input.txt");
    let (map, trailheads) = parsing(input);

    // for t in trailheads {
    //     println!("{:?}", t);
    // }

    let parts = count_parts(map, trailheads);
    println!("{:?}", parts);
}

fn count_parts(map: Vec<Vec<char>>, trailheads: Vec<(usize, usize)>) -> (i32, i32) {
    let trailhead_scores = get_trailhead_scores(&map, &trailheads);
    let part1: i32 = trailhead_scores.iter().sum();
    let trailhead_scores2 = get_trailhead_scores2(&map, &trailheads);
    let part2: i32 = trailhead_scores2.iter().sum();
    (part1, part2)
}

fn get_trailhead_scores2(map: &Vec<Vec<char>>, trailheads: &Vec<(usize, usize)>) -> Vec<i32>{
    let mut trailhead_scores:Vec<i32> = Vec::new();

    for trailhead in trailheads {
        let mut trail_ends: Vec<(usize, usize)> = Vec::new();
        let mut positions_list: Vec<(usize, usize)> = Vec::new();
        positions_list.push(trailhead.clone());

        while !positions_list.is_empty() {

            let current_pos = positions_list[0];
            positions_list.remove(0);

            let new_positions = get_new_positions(current_pos, &map);
           
            if new_positions.1 == 1 { // we found ends
                //add all ends?
                for new_pos in new_positions.0 {
                    trail_ends.push(new_pos.clone());
                }
            } else {
                //add all postions?
                for new_pos in new_positions.0 {
                    positions_list.push(new_pos.clone());
                }
            }
        }

        trailhead_scores.push(trail_ends.len() as i32);
    }


    trailhead_scores
}

fn get_trailhead_scores(map: &Vec<Vec<char>>, trailheads: &Vec<(usize, usize)>) -> Vec<i32>{
    let mut trailhead_scores:Vec<i32> = Vec::new();

    for trailhead in trailheads {
        let mut trail_ends: Vec<(usize, usize)> = Vec::new();
        let mut positions_list: Vec<(usize, usize)> = Vec::new();
        positions_list.push(trailhead.clone());

        while !positions_list.is_empty() {

            let current_pos = positions_list[0];
            positions_list.remove(0);

            let new_positions = get_new_positions(current_pos, &map);
           
            if new_positions.1 == 1 { // we found ends
                for new_pos in new_positions.0 {
                    if !trail_ends.contains(&new_pos) {
                        trail_ends.push(new_pos.clone());
                    }
                }
            } else {
                for new_pos in new_positions.0 {
                    if !positions_list.contains(&new_pos) {
                        positions_list.push(new_pos.clone());
                    }
                }
            }
        }

        trailhead_scores.push(trail_ends.len() as i32);
    }


    trailhead_scores
}

fn get_new_positions(position: (usize, usize), map: &Vec<Vec<char>>) -> (Vec<(usize, usize)>, i32) {
    let mut new_positions: Vec<(usize, usize)> = Vec::new();
    let mut end = 0;

    // Get the current position's number
    if let Some(position_num) = map[position.0][position.1].to_digit(10) {
        let new_pos_num = position_num + 1;

        for dir in DIRECTIONS {
            // Calculate new position with bounds checking
            let new_row = position.0 as isize + dir[0] as isize;
            let new_col = position.1 as isize + dir[1] as isize;

            if new_row >= 0 && new_row < map.len() as isize && new_col >= 0 && new_col < map[0].len() as isize {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if map[new_row][new_col].to_digit(10) == Some(new_pos_num) {
                    if new_pos_num == 9 {
                        end = 1;
                    }
                    new_positions.push((new_row, new_col));
                }
            }
        }
    }

    (new_positions, end)
}

fn parsing(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        map.push(chars);
    }

    let trailheads = get_trailheads(&map);

    (map, trailheads)
}

fn get_trailheads(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();

    for (x, line) in map.iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            if *c == '0' {
                trailheads.push((x, y));
            }
        }
    }


    trailheads
}