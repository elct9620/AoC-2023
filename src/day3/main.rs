use std::env;
use std::fs;
mod sum;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let part1 = sum::part1(contents.as_str());
    println!("Part 1: {}", part1);

    let part2 = sum::part2(contents.as_str());
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "#;

        let expected = 4361;
        let actual = sum::part1(input);
        assert_eq!(expected, actual);
    }
}
