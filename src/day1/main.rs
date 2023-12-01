use std::env;
use std::fs;
mod sum;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let part1 = sum::run(contents.as_str());
    println!("Part 1: {}", part1);

    let part2 = sum::run2(contents.as_str());
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = r#"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        "#;

        let expected = 142;
        let actual = sum::run(sample);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_letter() {
        let sample = r#"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        "#;

        let expected = 281;
        let actual = sum::run2(sample);
        assert_eq!(expected, actual);
    }
}
