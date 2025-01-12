fn main() {
    let input = include_str!("input.txt");  // Read the input file as a string
    let matrix = parsing(input);  // Call parsing to convert it into a matrix

    //part1
    let legit_list1 = count_valid_lists(&matrix);
    println!("{:?}", legit_list1);

    // part2
    // generate lists
    let all_lists = generate_lists(&matrix);
    let mut legit_list2 = 0;
    for outer_list in all_lists {
        let outer_list_validity = count_valid_lists(&outer_list);
        // println!("List: {:?} = {:?}", outer_list, outer_list_validity);
        legit_list2 += outer_list_validity;
    }


    println!("{:?}", legit_list2);
}

fn count_valid_lists(outer_list: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    for list in outer_list {
        let validity = is_valid(list);
        if validity == 1 {
            count += validity;
            break;
        }
    }

    count
}

fn is_valid(list: &Vec<i32>) -> i32 {
    let mut validity = 0;
    let mut ascension = true;

        for i in 0..list.len() - 1 {
            let first_num = list[i];
            let second_num = list[i + 1];

            // no ascension
            if first_num == second_num {
                break;
            }

            // set ascension on 1st iteration
            if first_num - second_num > 0 && i == 0 { //we are descending
                ascension = false;
            }

            //on other iterations check if the row continues in set direction
            if i != 0 {

                // if we are no longer ascending
                if first_num - second_num > 0 && ascension == true {
                    break;
                
                // if we are no longer descending
                } else if first_num - second_num < 0 && ascension == false { 
                    break;
                }
            }

            // set difference
            let difference = (first_num - second_num).abs(); 

            // if the jump is too big
            if difference < 1 || difference > 3 {
                break;
            }

            //end
            if (i + 1) == list.len() - 1 {
                validity += 1;
            }
        }

    validity
}

fn generate_lists(original_list: &Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
    let mut output_list: Vec<Vec<Vec<i32>>> = Vec::new();

    for line in original_list {
        let mut possible_lists: Vec<Vec<i32>> = Vec::new();

        // Push the original list as an InnerList
        possible_lists.push(line.clone());

        let line_length = line.len();
        for i in 0..line_length {
            let mut new_inner_list: Vec<i32> = Vec::new();

            for j in 0..line_length {
                if j != i {
                    new_inner_list.push(line[j]); // Add all except the i-th element
                }
            }

            possible_lists.push(new_inner_list);
        }

        output_list.push(possible_lists);
    }

    output_list
}


fn parsing(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()  // Split the input into lines
        .map(|line|   // For each line:
            line
                .split_whitespace()  // Split the line into space-separated parts
                .filter_map(|s| s.parse().ok())  // Convert each part into i32, filter out invalid parses
                .collect()  // Collect the results into a Vec<i32>
        )
        .collect()  // Collect all the Vec<i32> rows into a Vec<Vec<i32>>
}