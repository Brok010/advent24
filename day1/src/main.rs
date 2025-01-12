
fn main() {
    // Include the contents of the input.txt file as a string at compile time
    let input = include_str!("input.txt");
    let (list1, list2) = parsing(input);

    // println!("List 1: {:?}", list1);
    // println!("List 2: {:?}", list2);

    let mut part1 = 0;
    for i in 0..list1.len() {
        let mut temp = list1[i] - list2[i];
        temp = temp.abs();
        println!("{:?}", temp);
        part1 += temp;
    }


    let mut part2: i64 = 0;
    for i in 0..list1.len() {
        let left_num: i64 = list1[i];
        let mut times_counter: i64 = 0;

        for j in 0..list2.len() {
            if left_num == list2[j] {
                times_counter += 1;
            }
        }
        part2 += left_num * times_counter;
        println!("{:?} * {:?}", left_num, times_counter);
    }

    println!("{:?}", part1);
    println!("{:?}", part2);
}


fn parsing(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // println!("{:?}", parts)

        if parts.len() == 2 {
            if let (Ok(first), Ok(second)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                list1.push(first);
                list2.push(second);
            } else {
                eprintln!("Skipping line due to parsing error: {}", line);
            }
        } else {
            eprintln!("Skipping malformed line: {}", line);
        }
    }

    list1.sort();
    list2.sort();

    (list1, list2)
}
