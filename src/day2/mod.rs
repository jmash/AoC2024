use std::fs;
use std::path::Path;

fn check_if_row_is_inc_or_dec(row:&Vec<i32>) -> bool {
    let mut is_row_increasing:bool = true;
    let mut is_row_decreasing:bool = true;

    // first check if the row is all increasing
    for i in 0..row.len() - 1 {
        if row[i] >= row[i + 1] {
            is_row_increasing = false;
        }
    }

    // next check if the row is all decreasing
    for i in 0.. row.len() - 1 {
        if row[i] <= row[i + 1] {
            is_row_decreasing = false;
        }
    }

    // if either one or the other is true, we've got a safe row
    let is_row_inc_or_dec = is_row_increasing || is_row_decreasing;

    is_row_inc_or_dec
}

fn check_if_row_is_safe(row:&Vec<i32>) -> bool {

    // if check_if_row_is_inc_or_dec(&row) { println!("The row {:?} is safe for inc or dec", row); }
    // if check_if_row_is_properly_adjacent(&row) { println!("The row {:?} is safe for properly adjacent", row); }

    check_if_row_is_inc_or_dec(&row) && check_if_row_is_properly_adjacent(&row)
}

fn check_if_row_is_inc_or_dec_with_one_missing(row:&Vec<i32>) -> bool {
    dbg!(&row);
    let mut is_row_increasing:bool = true;
    let mut is_row_decreasing:bool = true;

    // if either one or the other is true, we've got a safe row
    let is_row_inc_or_dec = is_row_increasing || is_row_decreasing;

    is_row_inc_or_dec
}

fn check_if_row_is_properly_adjacent(row:&Vec<i32>) -> bool {
    let mut is_row_properly_adjacent:bool = true;

    for i in 0..row.len() - 1 {
        if (row[i + 1] - row[i]).abs() < 1 || (row[i + 1] - row[i]).abs() > 3 {
            is_row_properly_adjacent = false;
        }
    }

    is_row_properly_adjacent
}

pub fn day2_1() {
    // let input_path = Path::new("./src/inputs/day2.txt");
    let input_path = Path::new("./src/inputs/day2_test.txt");

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the puzzle input file.");

    // split each row by newline
    let report_rows = contents.trim().split('\n').collect::<Vec<&str>>();

    // split each string by whitespace to get individual characters
    let report_rows = report_rows.iter().map(|r| r.split_whitespace()
        .collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    // convert each character in each row into an i32
    // (I love how ugly this is)
    let report_rows =
        report_rows.iter()
        .map(|r| r.iter()
            .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();

    let mut safe_rows_count = 0;

    for row in report_rows {

        if check_if_row_is_safe(&row) {
            safe_rows_count += 1;
        }

    }

    dbg!(safe_rows_count);
}

pub fn day2_2() {
    let input_path = Path::new("./src/inputs/day2.txt");
    // let input_path = Path::new("./src/inputs/day2_test.txt");

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the puzzle input file.");

    // Process input to get rows of reports.

    // split each row by newline
    let report_rows = contents.trim().split('\n').collect::<Vec<&str>>();

    // split each string by newline to get individual characters
    let report_rows = report_rows.iter().map(|r| r.split_whitespace().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    // convert each character in each row into an i32
    // (I love how ugly this is)
    let report_rows = report_rows.iter().map(|r| r.iter().map(|s| s.parse().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    let mut safe_rows_count = 0;

    let mut row_count = 0;

    // go through each row and find out if it is SAFE on the first pass.

    for row in &report_rows {

        row_count += 1;
        // println!("Row Count: {}", row_count);

        // if the row is safe on the first pass, increase the safe_rows_count by 1.

        if check_if_row_is_safe(&row) {
            safe_rows_count += 1;
            println!("{:?}: Safe -> Passed Increase/Decrease: {}, Passed Proper Adjacency {}", row, check_if_row_is_inc_or_dec(&row), check_if_row_is_properly_adjacent(&row));
            // println!("Safe Rows on First Check {} on row {}", safe_rows_count, row_count);
        // BUT if t'weren't, then run the same check on the rows, where each row has one of its elements missing in sequence
        } else {

            // reset the row on each pass
            let origin_row = row.clone();

            let mut unsafe_flag = true;

            // remove the element in the ith index and test the modified row
            for i in 0.. row.len() {
                let mut modified_row = origin_row.clone();
                modified_row.remove(i);
                if check_if_row_is_safe(&modified_row) {
                    safe_rows_count += 1;
                    println!("{:?}: Safe -> Passed Increase/Decrease: {}, Passed Proper Adjacency {}", row, check_if_row_is_inc_or_dec(&modified_row), check_if_row_is_properly_adjacent(&modified_row));
                    unsafe_flag = false;

                    // println!("Safe Rows on Second Check {} on row {}", safe_rows_count, row_count);
                    break;
                }
            }
            if unsafe_flag { println!("{:?}: Unsafe", row); }
        }
    }

    dbg!(safe_rows_count);
}

// // first check if the row is all increasing
//     for i in 0..row.len() {
//         for j in 0..row.len() - 1 {
//             if i == j {
//                 continue;
//             }
//             if row[j] >= row[j + 1] {
//                 is_row_increasing = false;
//             }
//         }
//     }
//
// // next check if the row is all decreasing
//     for i in 0.. row.len() {
//         for j in 0.. row.len() - 1 {
//             if i == j {
//                 continue;
//             }
//             if row[j] <= row[j + 1] {
//                 is_row_decreasing = false;
//             }
//         }
//     }
