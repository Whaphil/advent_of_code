type Elf = Vec<usize>;

pub fn parse_input(unparsed_input: String) -> Vec<Elf> {
    let split_strings: Vec<&str> = unparsed_input.split("\n").collect();
    let split_unparsed_elfs = split_strings.split(|a| a == &"");
    let unparsed_elfs: Vec<&[&str]> = split_unparsed_elfs.collect();
    let parsed_elfs: Vec<Elf> = unparsed_elfs
        .iter()
        .map(|unparsed_elf| {
            unparsed_elf
                .iter()
                .map(|unparsed_value| unparsed_value.parse::<usize>().expect(""))
                .collect::<Elf>()
        })
        .collect::<Vec<Elf>>();
    return parsed_elfs;
}

pub fn solve(elfs: Vec<Elf>) -> String {
    let mut elfs_with_total_calories: Vec<usize> = elfs
        .iter()
        .map(|elf| {
            let mut total = 0;
            elf.iter().for_each(|value| total = total + value);
            return total;
        })
        .collect();

    elfs_with_total_calories.sort();
    elfs_with_total_calories.reverse();

    let top_three_elfs = &elfs_with_total_calories[..3];
    let elf_with_most_calories = top_three_elfs[0];

    let mut total_calories_of_top_three_elfs: usize = 0;
    top_three_elfs
        .iter()
        .for_each(|val| total_calories_of_top_three_elfs = total_calories_of_top_three_elfs + val);

    return format!(
        "\
The elf with the most calories carries {:?}
The top three elves are carrying {:?} calories
        ",
        elf_with_most_calories, total_calories_of_top_three_elfs
    );
}
