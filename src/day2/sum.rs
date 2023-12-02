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

    fn power(&self) -> u32 {
        return self.red * self.green * self.blue;
    }
}

pub fn part1(input: &str) -> u32 {
    let records = input
        .lines()
        .map(|line| to_game(line))
        .map(|game| game.ok())
        .filter(|game| match game {
            Some(game) => game.is_possible(12, 13, 14),
            None => false,
        })
        .map(|game| game.unwrap().id);

    return records.sum();
}

pub fn part2(input: &str) -> u32 {
    let records = input
        .lines()
        .map(|line| to_game(line))
        .filter(|game| match game {
            Ok(_) => true,
            Err(_) => false,
        })
        .map(|game| game.unwrap().power());

    return records.sum();
}

fn to_game(line: &str) -> Result<Game, ()> {
    let (id, actions) = extract(line)?;
    let mut game = Game::new(id);

    actions
        .iter()
        .map(|action| to_input(action))
        .map(|input| input.ok())
        .map(|input| input.unwrap())
        .for_each(|(count, color)| game.put(color, count));

    return Ok(game);
}

fn extract(record: &str) -> Result<(u32, Vec<&str>), ()> {
    let mut parts = record.split(":");

    let id = parts.next().and_then(to_id).ok_or(())?;
    let actions = parts.next().and_then(to_actions).ok_or(())?;

    return Ok((id, actions));
}

fn to_id(input: &str) -> Option<u32> {
    return input.trim().split(" ").nth(1).and_then(to_number);
}

fn to_actions(input: &str) -> Option<Vec<&str>> {
    return Some(
        input
            .trim()
            .split(";")
            .flat_map(|action| action.split(","))
            .map(str::trim)
            .collect::<Vec<&str>>(),
    );
}

fn to_input(input: &str) -> Result<(u32, &str), ()> {
    let mut parts = input.split(" ");
    let count = parts.next().and_then(to_number).ok_or(())?;
    let color = parts.next().ok_or(())?;
    return Ok((count, color));
}

fn to_number(input: &str) -> Option<u32> {
    return input.trim().parse::<u32>().ok();
}
