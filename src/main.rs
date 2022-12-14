pub mod _template;
pub mod day01;
pub mod day02;

use std::fs;

pub fn solve<T>(path_to_folder: String, parse_input: fn(String) -> T, _solve: fn(T) -> String) {
    let unparsed_input = read_input(path_to_folder.clone() + "/input.txt");
    let parsed_input = parse_input(unparsed_input);
    let solution = _solve(parsed_input);
    write_solution(solution, path_to_folder + "/output.txt")
}

fn read_input(path: String) -> String {
    let unchecked_input = fs::read_to_string(path.clone());
    let input = match unchecked_input {
        Ok(val) => val,
        Err(err) => panic!("Something went wrong trying to read file {}: {}", path, err),
    };
    return input;
}

fn write_solution(solution: String, path: String) -> () {
    let write_result = fs::write(path, solution);
    match write_result {
        Ok(_) => (),
        Err(err) => panic!("Something went wrong trying to write the solution: {}", err),
    }
}

fn main() {
    solve(
        "./src/_template".to_owned(),
        _template::parse_input,
        _template::solve,
    );
    solve("./src/day01".to_owned(), day01::parse_input, day01::solve);
    solve("./src/day02".to_owned(), day02::parse_input, day02::solve)
}
