mod sum;

fn main() {
    let res = sum::run("");
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
