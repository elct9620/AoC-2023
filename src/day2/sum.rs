use std::str::Split;

struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new(id: u32) -> Game {
        return Game {
            id,
            red: 0,
            green: 0,
            blue: 0,
        };
    }

    fn put(&mut self, color: &str, count: u32) {
        match color {
            "red" => self.red = count.max(self.red),
            "green" => self.green = count.max(self.green),
            "blue" => self.blue = count.max(self.blue),
            _ => (),
        }
    }

    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        return self.red <= red && self.green <= green && self.blue <= blue;
    }
}

pub fn part1(input: &str) -> u32 {
    let records = input
        .lines()
        .map(|line| decode_to_game(line))
        .map(|game| game.ok())
        .filter(|game| match game {
            Some(game) => game.is_possible(12, 13, 14),
            None => false,
        })
        .map(|game| game.unwrap().id);

    return records.sum();
}

fn decode_to_game(line: &str) -> Result<Game, ()> {
    let (id, actions) = extract_record(line)?;
    let mut game = Game::new(id);
    for action in actions {
        let inputs = action.split(",");
        for input in inputs {
            let (count, color) = decode_input(input.trim());
            game.put(color, count);
        }
    }

    return Ok(game);
}

fn extract_record(record: &str) -> Result<(u32, Split<'_, &str>), ()> {
    let parts = record.split(":").collect::<Vec<&str>>();
    let id = match parts[0].trim().split(" ").nth(1) {
        Some(id) => id.parse::<u32>().unwrap(),
        None => return Err(()),
    };
    let actions = parts[1].split(";");

    return Ok((id, actions));
}

fn decode_input(input: &str) -> (u32, &str) {
    let count = input.split(" ").nth(0).unwrap().parse::<u32>().unwrap();
    let color = input.split(" ").nth(1).unwrap();
    return (count, color);
}
