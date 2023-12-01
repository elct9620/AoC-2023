pub fn run(input: &str) -> i32 {
    let calibration_values = input.lines().map(find_calibration_value);

    return calibration_values.sum();
}

fn find_calibration_value(input: &str) -> i32 {
    let left_num_idx = input.find(|c: char| c.is_digit(10));
    let right_num_idx = input.rfind(|c: char| c.is_digit(10));

    if left_num_idx.is_none() || right_num_idx.is_none() {
        return 0;
    }

    let left_num_char = input.chars().nth(left_num_idx.unwrap()).unwrap();
    let right_num_char = input.chars().nth(right_num_idx.unwrap()).unwrap();
    let num = format!("{}{}", left_num_char, right_num_char)
        .parse::<i32>()
        .unwrap();

    return num;
}
