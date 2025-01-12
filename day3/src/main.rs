use std::{f32::consts::E, str::FromStr};

#[derive(Debug)]
struct Mul {
    start_index: usize,
    num1: i32,
    num2: i32,
    end_index: usize,
}

#[derive(Debug)]
struct DosDonts {
    start_index: usize,
    validity: bool,
    end_index: usize
}

fn main() {
    let input = include_str!("input.txt"); // Read the input file as a string
    let (mul_list, do_list) = parsing(input);

    // Print the mul_list
    for mul in &mul_list {
        println!("{:?}", mul);
    }

    // Print the mul_list
    for each in &do_list {
        println!("{:?}", each);
    }

    // part 1
    let part1 = count_part1(&mul_list);
    println!("Part1: {:?}", part1);


    //part2
    let part2 = count_part2(&mul_list, &do_list);
    println!("Part2: {:?}", part2);
}

fn count_part2(muls: &Vec<Mul>, dos: &Vec<DosDonts>) -> i32 {
    let mut part2 = 0;
    let mut j = 0;
    let mut count = true;

    for mul in muls {
        while j < dos.len() - 1 && mul.start_index > dos[j].start_index {
            count = dos[j].validity;
            j += 1;
        }

        if mul.start_index > dos[j].start_index { //happens only in last cases
            count = dos[j].validity;
        }

        if count == true {
            part2 += mul.num1 * mul.num2;
        }
    }

    part2
}

fn count_part1(list: &Vec<Mul>) -> i64 {
    let mut part1: i64 = 0;

    for each in list {
        part1 += each.num1 as i64 * each.num2 as i64;
    }

    part1
}


fn parsing(input: &str) -> (Vec<Mul>, Vec<DosDonts>) {
    let mut mul_list: Vec<Mul> = Vec::new();
    let mut do_list: Vec<DosDonts> = Vec::new();
    let mut mul_iterator_index = 0;

    while mul_iterator_index < input.len() {
        match find_next_mul_struct(mul_iterator_index, input) {
            Ok(new_mul_struct) => {
                mul_iterator_index = new_mul_struct.end_index; // Advance by the end_index
                mul_list.push(new_mul_struct);
            }
            Err(next_index) => {
                mul_iterator_index = next_index; // Advance to the next index if parsing fails
            }
        }
    }

    let mut do_iterator_index = 0;
    while do_iterator_index < input.len() {
        let new_do_struct = find_next_do_struct(do_iterator_index, input);
        do_iterator_index = new_do_struct.end_index; // Advance by the end_index
        do_list.push(new_do_struct);
    }

    // last do struct is allways default cause of error handling
    do_list.pop();

    (mul_list,do_list)
}

fn find_next_mul_struct(start_index: usize, input: &str) -> Result<Mul, usize> {
    let searched_string = &input[start_index..];

    if let Some(mul_index) = searched_string.find("mul") {
        let actual_mul_index = start_index + mul_index;
        if let Some(start_par_index) = input[actual_mul_index..].find('(') {
            let actual_start_par_index = actual_mul_index + start_par_index;
            if actual_start_par_index == actual_mul_index + 3 {
                if let Some(end_par_index) = input[actual_start_par_index..].find(')') {
                    let actual_end_par_index = actual_start_par_index + end_par_index;
    
                    // Extract and parse the numbers inside the parentheses
                    let parse = &input[actual_start_par_index + 1..actual_end_par_index];
                    // println!("{:?}", parse);
                    let integers: Vec<&str> = parse.split(',').collect();
                    if integers.len() == 2 {
                        if let (Ok(num1), Ok(num2)) = (i32::from_str(integers[0]), i32::from_str(integers[1])) {
                            return Ok(Mul {
                                start_index: actual_mul_index,
                                num1,
                                num2,
                                end_index: actual_end_par_index,
                            });
                        }
                    }
                }
            }
        }

        // Parsing failed, move to next character after "mul"
        return Err(actual_mul_index + 1);
    }

    // No "mul" found, stop parsing
    Err(input.len()) // Set the index to input.len() to terminate the loop
}

fn find_next_do_struct(start_index: usize, input: &str) -> DosDonts {
    let searched_string = &input[start_index..];
    let mut validity = true;

    if let Some(do_index) = searched_string.find("do") {
        let actual_do_index = start_index + do_index;
        let mut actual_end_index = actual_do_index + 2;
        // println!("{:?}", &searched_string[do_index + 2..do_index + 5]);
        if &searched_string[do_index + 2..do_index + 5] == "n't" {
            validity = false;
            actual_end_index += 3;
        }

        return DosDonts {
            start_index: actual_do_index,
            validity: validity,
            end_index: actual_end_index
        };
    }
    // Return a default value indicating no match was found
    DosDonts {
        start_index: input.len(),
        validity: false,
        end_index: input.len(),
    }
}