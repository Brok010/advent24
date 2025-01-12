#[derive(PartialEq)]
#[derive(Clone, Debug)]
struct Anthena {
    x: i32,
    y: i32,
    c: char
}

fn main() {
    let input = include_str!("input.txt");
    let (anthenas, x_axis_length, y_axis_length) = parsing(input);

    let part1 = count_part_one(anthenas, x_axis_length, y_axis_length);
    
    println!("{}", part1);
}

fn count_part_one(anthenas: Vec<Anthena>, x_max: i32, y_max: i32) -> i32 {
    let mut processed_frequencies: Vec<char> = Vec::new();
    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    // for each new frequency find all anthenas at that frequency and process them
    for first_anthena in &anthenas {
        if !processed_frequencies.contains(&first_anthena.c) {
            processed_frequencies.push(first_anthena.c.clone());
            let mut current_anthenas: Vec<Anthena> = Vec::new();
            current_anthenas.push(first_anthena.clone());

            for other_anthena in &anthenas {
                if other_anthena.c == first_anthena.c &&
                   (other_anthena.x != first_anthena.x ||
                   other_anthena.y != first_anthena.y) {
                    current_anthenas.push(other_anthena.clone());
                   }
            }

            //proces found anthenas
            if current_anthenas.len() > 1 {
                let (new_antinodes_part1, new_antinodes_part2) = get_antinodes(current_anthenas, x_max, y_max);

                for ant in new_antinodes_part1 {
                    if !antinodes.contains(&ant) {
                        antinodes.push(ant);
                    }
                }
            }
        }
    }

    // for a in &antinodes {
    //     println!("{:?}", a);
    // }

    return antinodes.len() as i32;
}

fn get_antinodes(anthenas: Vec<Anthena>, x_max: i32, y_max: i32) -> (Vec<(i32, i32)>, Vec<(i32, i32)>){
    let mut antinodes1: Vec<(i32, i32)> = Vec::new();
    let mut antinodes2: Vec<(i32, i32)> = Vec::new();
    
    for i in 0..anthenas.len() {
        for j in (i + 1)..anthenas.len() {
            let anthena1 = &anthenas[i];
            let anthena2 = &anthenas[j];

            let kx = anthena1.x - anthena2.x;
            let ky = anthena1.y - anthena2.y;

            let steps = gcd(kx, ky); //greatest common divisor
            
            if steps == 1 {

                let mut new_x1 = anthena1.x;
                let mut new_x2 = anthena2.x;
                let mut new_y1 = anthena1.y;
                let mut new_y2 = anthena2.y;
                
                //part2
                while  new_x1 > 0 && new_x1 < x_max && new_y1 > 0 && new_y1 < y_max ||
                       new_x2 > 0 && new_x2 < x_max && new_y2 > 0 && new_y2 < y_max {
                    
                        continue;
                }


                //part1
                if kx < 0 { // anthena1x < anthena2x
                    new_x1 = anthena2.x + kx.abs(); //check
                    new_x2 = anthena1.x - kx.abs();
                } else if kx > 0 { // if its 0 its already set
                    new_x1 = anthena2.x - kx.abs(); //check
                    new_x2 = anthena1.x + kx.abs();
                }

                if ky < 0 { // anthena1y < anthena2y
                    new_y1 = anthena2.y + ky.abs(); //check
                    new_y2 = anthena1.y - ky.abs();
                } else if ky > 0 { // if its 0 its already set
                    new_y1 = anthena2.y - ky.abs(); //prob
                    new_y2 = anthena1.y + ky.abs();
                }
                antinodes1.push((new_x1, new_y1));
                antinodes1.push((new_x2, new_y2));
            }
            
            if steps == 3 { //for part 2 this doesnt change
                let mut new_x1 = anthena1.x;
                let mut new_y1 = anthena2.x;
                let mut new_x2 = anthena1.y;
                let mut new_y2 = anthena2.y;

                if kx < 0 {
                    new_x1 = anthena1.x + kx.abs();
                    new_x2 = anthena2.x - kx.abs();
                } else if kx > 0 { // if its 0 its already set
                    new_x1 = anthena2.x + kx.abs();
                    new_x2 = anthena1.x - kx.abs();
                }

                if ky < 0 {
                    new_y1 = anthena1.y + ky.abs();
                    new_y2 = anthena2.y - ky.abs();
                } else if ky > 0 { // if its 0 its already set
                    new_y1 = anthena2.y + ky.abs();
                    new_y2 = anthena1.y - ky.abs();
                }
                antinodes1.push((new_x1, new_y1));
                antinodes1.push((new_x2, new_y2));
            }
            //other cases dont have relevant antinodes
        }
    }
    antinodes1.retain(|&(x, y)| x >= 0 && x <= x_max - 1 && y >= 0 && y <= y_max - 1);
    (antinodes1, antinodes2)
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

fn parsing(input: &str) -> (Vec<Anthena>, i32, i32){
    let mut x_axis = 0;
    let mut y_axis = 0;
    let mut anthenas: Vec<(Anthena)> = Vec::new();
    let mut map: Vec<Vec<char>> = Vec::new();
    
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        map.push(chars);
    }

    x_axis = map[0].len() as i32;
    y_axis = map.len() as i32;

    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != '.' {
                let x = j as i32;
                let y = i as i32;
                let ch = *c;
                anthenas.push(Anthena{ x, y, c:ch});
            }
        }
    }

    (anthenas, x_axis, y_axis)
}
