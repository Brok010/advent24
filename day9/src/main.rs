#[derive(Clone, Debug)] 
struct Xo {
    list_index: i32,
    file_index: i32
}

fn main() {
    let input = include_str!("input.txt");
    let xos = parsing(input);

    let part1 = solve_part1(&xos);
    let part2 = solve_part2(xos);
    println!("{}, {}", part1, part2);
}

fn solve_part2(list: Vec<Xo>) -> i64 {
    let new_disk = get_sorted_disk_for_part2(list);
    let part2 = count_disk_part2(new_disk);
    part2
}

fn count_disk_part2(disk: Vec<Xo>) -> i64 {
    let mut counter: i64 = 0;

    for (i, each) in disk.iter().enumerate() {
        if each.file_index != -1 {
            counter += i as i64 * each.file_index as i64;
        }
    }

    counter
}

fn get_sorted_disk_for_part2(mut list: Vec<Xo>) -> Vec<Xo> {
    // printlist(&list);
    
    let mut i = list.len() - 1;
    while i > 0 {
        if list[i].file_index != -1 {
            let file_size = get_file_size(i, &list);

            let mut j = 0;
            while j < list.len() - 1 && (j as i32) < i as i32 - file_size {
                if list[j].file_index == -1 {
                    let mut space_found = false;
                    let space_size = get_space_size(j, &list);
                    if space_size >= file_size {
                        for k in 0..file_size {
                            // Change values
                            let temp = list[i - k as usize].clone();
                            list[i - k as usize] = list[j + k as usize].clone();
                            list[j + k as usize] = temp;

                            // printlist(&list);
                        }
                        space_found = true;
                    }

                    j += space_size as usize; // Skip `space_size` iterations
                    if space_found {
                        break;
                    }
                } else {
                    j += 1; // Move to the next iteration
                }
            }

            if i as i32 - file_size < 0 {
                break; // Break the loop if `i` goes out of bounds
            } else {
                i -= file_size as usize; // Manually decrement `i` by `file_size`
            }
        } else {
            if i == 0 {
                break; // Ensure we don't underflow
            }
            i -= 1; // Move to the previous element
        }
    }


    list
}

fn printlist(list: &Vec<Xo>) {
    for each in list {
        if each.file_index != -1 {
            print!("{}", each.file_index);
        } else {
            print!(".");
        }
    }
    println!();
}

fn get_space_size(i: usize, list: &Vec<Xo>) -> i32 {
    let mut j = 1;
    while i+j < list.len() - 1 {
        if list[i+j].file_index == -1 {
            j += 1;
        } else { break; }
    } 
    j as i32
}

fn get_file_size(i: usize, list: &Vec<Xo>) -> i32 {
    let file_index = list[i].file_index;
    let mut j = 1;
    let ii32 = i as i32;
    let mut ji32 = j as i32;

    while ii32 - ji32 < (list.len() - 1) as i32 && ii32 - ji32 >= 0 {
        if list[i-j].file_index == file_index {
            j += 1;
            ji32 = j as i32;
        } else { break; }
    } 
    j as i32
}

fn solve_part1(list: &Vec<Xo>) -> i64 {
    let new_disk = get_sorted_disk_for_part1(list);
    let part1 = count_disk(new_disk);
    part1
}   

fn count_disk(disk: Vec<Xo>) -> i64 {
    let mut counter: i64 = 0;

    for each in disk {
        if each.file_index != -1 {
            counter += each.file_index as i64 * each.list_index as i64;
        }
    }

    counter
}

fn get_sorted_disk_for_part1(list: &Vec<Xo>) -> Vec<Xo> {
    let mut sorted_disk: Vec<Xo> = Vec::new();
    let mut contra_counter = list.len() - 1;

    for i in 0..list.len() {
        // println!("{:?}", list[i]);

        if i >= contra_counter && list[i].file_index == -1 { break; } else if i >= contra_counter {
            sorted_disk.push(list[i].clone());
            break;
        }

        if list[i].file_index != -1 {
            sorted_disk.push(list[i].clone());
        } else {
            while contra_counter > i {
                if list[contra_counter].file_index != -1 {
                    let new_xo = Xo {list_index: i as i32, file_index: list[contra_counter].file_index};
                    sorted_disk.push(new_xo);
                    contra_counter -= 1;
                    break;
                } else {
                    contra_counter -= 1;
                }
            }
        }
    }

    sorted_disk
}

fn parsing(input: &str) -> Vec<Xo> {
    let mut xos = Vec::new();
    let mut free = false;
    let mut file_index = 0;
    let mut list_index = -1;

    for c in input.chars().filter(|c| c.is_ascii_digit()) {
        let loop_counter = c.to_digit(10).unwrap();

        // there are no 0byte files in input
        // if loop_counter == 0 && free == false {
        //     println!("error")
        // }
        
        if free {
            file_index += 1;
        }

        for _ in 0..loop_counter {
            list_index += 1;
            let new_file_index = if free { -1 } else { file_index };

            xos.push( Xo {
                list_index,
                file_index: new_file_index,
            });
        }

        free = !free; // Toggle `free`
    }

    xos
}
