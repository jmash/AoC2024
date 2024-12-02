fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn get_newline_count(s: &str) -> u32 {
    s.chars().filter(|c| *c == '\n').count() as u32
}

pub fn day1_1() {
    use std::fs;
    use std::path::Path;

    // let input_path = Path::new("./src/inputs/day1_test.txt");
    let input_path = Path::new("./src/inputs/day1.txt");

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the puzzle input file.");

    let contents = contents.trim().split_whitespace().collect::<Vec<&str>>();

    let contents = contents.iter().map(|s| s.parse().unwrap()).collect::<Vec<i32>>();

    let mut left_list = vec![0; 0];
    let mut right_list = vec![0; 0];

    for i in 0..contents.len() {
        if i % 2 == 0 {
            left_list.push(contents[i])
        } else {
            right_list.push(contents[i]);
        }
    }

    left_list.sort();
    right_list.sort();

    let mut differences_list = vec![0; 0];

    for i in 0..left_list.len() {
        let difference = (left_list[i] - right_list[i]).abs();
        differences_list.push(difference);
    }

    let differences_sum = differences_list.iter().sum::<i32>();

    dbg!(differences_sum);
}

pub fn day1_2() {
    use std::fs;
    use std::path::Path;

    // let input_path = Path::new("./src/inputs/day1_test.txt");
    let input_path = Path::new("./src/inputs/day1.txt");

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the puzzle input file.");

    let contents = contents.trim().split_whitespace().collect::<Vec<&str>>();

    let contents = contents.iter().map(|s| s.parse().unwrap()).collect::<Vec<i32>>();

    let mut left_list = vec![0; 0];
    let mut right_list = vec![0; 0];

    for i in 0..contents.len() {
        if i % 2 == 0 {
            left_list.push(contents[i])
        } else {
            right_list.push(contents[i]);
        }
    }

    let mut frequency_totals:Vec<i32> = vec![0, 0];

    for i in 0..left_list.len() {
        let mut frequency_count:i32 = 0;
        for j in 0..right_list.len() {
            if left_list[i] == right_list[j] {
                frequency_count = frequency_count + 1;
            }
        }
        frequency_totals.push(left_list[i] * frequency_count);
    }

    let frequency_total = frequency_totals.iter().sum::<i32>();

    dbg!(frequency_total);
}


// let pair_count = get_newline_count(contents.as_str());
// let pair_count = pair_count + 1;
//
// println!("Number of pairs: {}", pair_count);
//
// let contents = contents.trim();
//
// let contents = remove_whitespace(contents);
//
// let contents = contents.trim().chars().as_str();
//
// let mut left_list = vec![0; pair_count as usize];
// let mut right_list = vec![0; pair_count as usize];
//
// let mut left_index:usize = 0;
// let mut right_index:usize = 0;
//
// // there is almost certainly a one-liner that would do this, but I am tired right now.
// for i in 0..pair_count*2 {
// if i % 2 == 0 {
// left_list[left_index] = contents.as_bytes()[i as usize] as u32;
// left_index = left_index + 1
// } else {
// right_list[right_index] = contents.as_bytes()[i as usize] as u32;
// right_index = right_index + 1
// }
// }
//
// println!("Left list: {:?}", left_list);
// println!("Right list: {:?}", right_list);
//
// left_list.sort();
// right_list.sort();
//
// println!("Left list sorted: {:?}", left_list);
// println!("Right list sorted: {:?}", right_list);
//
// let mut difference_list = vec![0; 0];
//
// for i in 0..pair_count {
// let difference = (left_list[i as usize] as i32 - right_list[i as usize] as i32).abs();
// difference_list.push(difference);
// }
//
// println!("Difference list: {:?}", difference_list);
//
// let total:i32 = difference_list.iter().sum();
//
// println!("Total: {}", total);