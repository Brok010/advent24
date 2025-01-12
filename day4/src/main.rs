
fn main() {
    let input = include_str!("input.txt"); // Read the input file as a string
    let char_list = parsing(&input);

    // for line in char_list {
    //     println!("{:?}", line);
    // }
    // find every x and try to make xmass in all directions, count directions
    let part1 = get_xmas_count(&char_list);
    println!("{}", part1);

    let part2 = get_mas_count(&char_list);
    println!("{}", part2);
}

fn get_mas_count(char_list: &Vec<Vec<char>>) -> i32 {
    let mut mas_counter = 0;

    for (i, line) in char_list.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'A' {
                mas_counter += posible_mas_from_a( i as i32, j as i32, &char_list);
                println!("{}, {}, {}", i, j, mas_counter);
            }
        }
    }

    mas_counter
}

fn posible_mas_from_a(x: i32, y: i32, char_list: &Vec<Vec<char>>) -> i32 {
    let mut counter = 0;
    let directions = [[1, 1], [-1, -1], [-1, 1], [1, -1]];

    let mut letter_list: Vec<char> = Vec::new();
    for d in directions {
        let new_x = x + d[0];
        let new_y = y + d[1];

        if new_x < 0 || new_y < 0 || new_y > char_list.len() as i32 - 1|| new_x > char_list[0].len() as i32 - 1 {
            break;
        } else {
            letter_list.push(char_list[new_x as usize][new_y as usize]);
        }

    }

    let m_count = letter_list.iter().filter(|&&c| c == 'M').count();
    let s_count = letter_list.iter().filter(|&&c| c == 'S').count();
    // println!("{:?}", letter_list);
    if m_count == 2 && s_count == 2 && letter_list[0] != letter_list[1] {
        counter += 1;
    }

    counter
}

fn get_xmas_count(char_list: &Vec<Vec<char>>) -> i32 {
    let mut xmas_counter = 0;

    for (i, line) in char_list.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'X' {
                xmas_counter += posible_xmas_from_x( i as i32, j as i32, &char_list);
                // println!("{}, {}, {}", i, j, xmas_counter);
            }
        }
    }
    
    xmas_counter
}

fn posible_xmas_from_x(x: i32, y: i32, char_list: &Vec<Vec<char>>) -> i32 {
    let mut counter = 0;
    let directions = [[-1, 0],[1, 0],[0, -1],[0, 1], [1, 1], [1, -1], [-1, 1], [-1, -1]];
    let chars = ['M', 'A', 'S'];

    for d in directions {
        let mut sequence_counter = 1;

        for c in chars {
            let new_x = x + d[0] * sequence_counter;
            let new_y = y + d[1] * sequence_counter;
        
            if new_x < 0 || new_y < 0 || new_y > char_list.len() as i32 - 1|| new_x > char_list[0].len() as i32 - 1 {
                break;
            } else if char_list[new_x as usize][new_y as usize] != c {
                // println!("{}, {}", char_list[new_x as usize][new_y as usize], c);
                break;
            } else if char_list[new_x as usize][new_y as usize] == c && c == 'S' {
                counter += 1
            } else if char_list[new_x as usize][new_y as usize] == c && c != 'S' {
                sequence_counter += 1;
            }
        }  
    }
    counter
}

fn parsing(input: &str) -> Vec<Vec<char>> {
    let mut char_list: Vec<Vec<char>> = Vec::new();

    for line in input.lines() { // Use .lines() to iterate over lines
        let chars: Vec<char> = line.chars().collect(); // Use .chars() to iterate over characters
        char_list.push(chars);
    }

    char_list
}
