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
