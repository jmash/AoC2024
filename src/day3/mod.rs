use std::fs;
use std::path::Path;

fn read_input() -> String {
    // let input_path = Path::new("./src/inputs/day3.txt");
    let input_path = Path::new("./src/inputs/day3_test.txt");

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the puzzle input file.");

    contents
}

const TOKEN_M:char = 'm';
const TOKEN_U:char = 'u';
const TOKEN_L:char = 'l';
const TOKEN_LEFT_PAREN:char = '(';
const TOKEN_COMMA:char = ',';
const TOKEN_RIGHT_PAREN:char = ')';

struct ParserNode {
    compare_char: char,
    next_node: ParserNode,
}

impl ParserNode {
    fn new(compare_char: char, next_node: ParserNode) -> Self {
        Self {
            compare_char,
            next_node
        }
    }

    fn traverse(&mut self, input_char:char, compare_char:char) {
        if compare_char == input_char {

        }
    }
}

pub fn day3_1() {
    let puzzle_input = read_input();
    let mut puzzle_input = puzzle_input.chars().peekable();

    let mut valid_stack = "".to_string();

    loop {

    }
}

pub fn day3_2() {

}