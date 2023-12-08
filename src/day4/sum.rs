pub fn part1(input: &str) -> u32 {
    let cards = to_cards(input);
    cards.iter().map(|c| c.score()).sum::<u32>()
}

pub fn part2(_input: &str) -> u32 {
    0
}

struct Card {
    expected: Vec<u32>,
    owned: Vec<u32>,
}

impl Card {
    fn new(expected: Vec<u32>, owned: Vec<u32>) -> Card {
        Card { expected, owned }
    }

    fn matches(&self) -> Vec<u32> {
        self.expected
            .iter()
            .filter(|&v| self.owned.contains(v))
            .map(|v| *v)
            .collect()
    }

    fn score(&self) -> u32 {
        let count = self.matches().len() as u32;
        if count > 0 {
            return 2u32.pow(count.saturating_sub(1));
        };

        0
    }
}

fn to_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| extract_numbers(line))
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .map(|(expected, owned)| Card::new(expected, owned))
        .collect()
}

fn extract_numbers(input: &str) -> Result<(Vec<u32>, Vec<u32>), ()> {
    let mut parts = input.split(":");
    let mut numbers = parts.nth(1).ok_or(())?.split("|");
    let expected = parse_numbers(numbers.next().unwrap());
    let owned = parse_numbers(numbers.next().unwrap());

    Ok((expected, owned))
}

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .filter(|v| *v > 0)
        .collect()
}
