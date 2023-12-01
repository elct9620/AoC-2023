use std::env;
use std::fs;
mod sum;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let res = sum::run(contents.as_str());
    println!("Sum: {}", res);
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
}
