use std::fs;

pub fn solve(path_to_folder: String) {
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
    println!("{}", input);
    return input;
}

fn write_solution(solution: String, path: String) -> () {
    let write_result = fs::write(path, solution);
    match write_result {
        Ok(_) => (),
        Err(err) => panic!("Something went wrong trying to write the solution: {}", err),
    }
}

fn parse_input(unparsed_input: String) -> String {
    return unparsed_input;
}

fn _solve(formatted_input: String) -> String {
    return formatted_input.to_string();
}
