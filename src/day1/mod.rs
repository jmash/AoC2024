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