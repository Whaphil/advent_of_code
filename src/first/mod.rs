use core::panic;

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
    let elfs_with_total_calories: Vec<usize> = elfs
        .iter()
        .map(|elf| {
            let mut total = 0;
            elf.iter().for_each(|value| total = total + value);
            return total;
        })
        .collect();

    let elf_with_most_calories =
        elfs_with_total_calories
            .iter()
            .reduce(|prev, curr| match prev >= curr {
                true => prev,
                false => curr,
            });

    match elf_with_most_calories {
        Some(val) => val.to_string(),
        None => panic!("FOUND NO ELF WITH MOST CALORIES"),
    }
}
