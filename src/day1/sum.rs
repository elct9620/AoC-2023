pub fn run(input: &str) -> i32 {
    let calibration_values = input.lines().map(find_calibration_value);

    return calibration_values.sum();
}

fn find_calibration_value(input: &str) -> i32 {
    let digits: Vec<&str> = input.matches(char::is_numeric).collect();
    let head = digits.first().unwrap_or(&"0");
    let tail = digits.last().unwrap_or(&"0");

    let num = format!("{}{}", head, tail).parse::<i32>().unwrap();

    return num;
}

/**
 * Reference: https://www.reddit.com/r/adventofcode/comments/1883ibu/2023_day_1_solutions/
 * The word may overlap e.g. "one1eightwo" -> 12
 */
pub fn run2(input: &str) -> i32 {
    let calibration_values = input.lines().map(find_calibration_value2);

    return calibration_values.sum();
}

const LETTERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_calibration_value2(input: &str) -> i32 {
    let len = input.len();
    let head = (0..len).find_map(|i| compare(&input[i..])).unwrap_or(0);
    let tail = (0..len)
        .rev()
        .find_map(|i| compare(&input[i..]))
        .unwrap_or(0);

    return (10 * head + tail) as i32;
}

fn compare(slice: &str) -> Option<u32> {
    LETTERS
        .iter()
        .enumerate()
        .find(|(_, &word)| slice.starts_with(word))
        .map(|(i, _)| (i + 1) as u32)
        .or_else(|| slice.chars().next().unwrap().to_digit(10))
}
