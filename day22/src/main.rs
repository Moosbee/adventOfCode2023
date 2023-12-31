use std::fs;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());
    println!("Solution: {} Took {:?}", 0, start.elapsed())
}
