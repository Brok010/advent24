const DIRECTIONS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, 1], [0, -1]];


fn main() {
    let input = include_str!("input.txt");
    let map = parsing(input);
    let regions = get_regions(map);

    for r in &regions {
        println!("{:?}", r);
    }

    let part1 = solve_p1(regions);
    println!("{}", part1);
}

fn solve_p1(data: Vec<(i32, i32)>) -> i32 {
    let mut result = 0;
    for each in data {
        result += each.0 * each.1;
    }

    result
}

fn get_regions(map: Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut checked_for_positions: Vec<(usize, usize)> = Vec::new();
    let mut regions: Vec<(i32, i32)> = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            
            let current_pos = (x, y);

            // for pos in &checked_for_positions {
            //     print!("{:?}", pos);
            // }
            // println!();

            if !checked_for_positions.contains(&current_pos) {
                checked_for_positions.push(current_pos.clone());

                let current_sign = map[y][x];
                let (area, walls, new_checked) = get_regions_for_sign(&map, current_pos, current_sign, checked_for_positions.clone());
                
                // for nc in &new_checked {
                //     println!("{:?}", nc);
                // }



                for nc in new_checked {
                    if !checked_for_positions.contains(&nc) {
                        checked_for_positions.push(nc);
                    }
                }
                // println!("{}, {}", area, walls);
                regions.push((area, walls));
            }
        }
    }

    regions
}

fn get_regions_for_sign(map: &Vec<Vec<char>>, pos: (usize, usize), sign: char, mut checked_positions: Vec<(usize ,usize)>) -> (i32, i32, Vec<(usize, usize)>) {

    // for x in &checked_positions {
    //     println!("{:?}", x);
    // }

    let mut area = 1;
    let mut walls = 0;
    let map_x = map.len() as i32;
    let map_y = map[0].len() as i32;

    for dir in DIRECTIONS {
        let new_pos = get_new_pos(&pos, dir);

        if (new_pos.0 >= 0 && new_pos.0 < map_x) && (new_pos.1 >= 0 && new_pos.1 < map_y) {
            if map[new_pos.1 as usize][new_pos.0 as usize] == sign {
                let usize_pos = (new_pos.0 as usize, new_pos.1 as usize);
                
                if !checked_positions.contains(&usize_pos) {
                    checked_positions.push(usize_pos.clone());

                    let (n_area, n_walls, n_checked) = get_regions_for_sign(map, usize_pos, sign, checked_positions.clone());

                    for nc in n_checked {
                        if !checked_positions.contains(&nc) {
                            checked_positions.push(nc);
                        }
                    }

                    area += n_area;
                    walls += n_walls;
                }
            } else {
                walls += 1;
            }


        } else {
            walls += 1;
        }
    }    

    (area, walls, checked_positions)
}

fn get_new_pos(pos: &(usize, usize), dir: [i32; 2]) -> (i32, i32) {
    let x:i32 = pos.0 as i32 + dir[0];
    let y:i32 = pos.1 as i32 + dir[1];
    (x, y)
}

fn parsing(input: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        map.push(chars);
    }
    map
}