use std::str;

pub fn part1(input: &str) -> u32 {
    let records = input.lines().map(|line| possible_game_id(line));
    return records.sum();
}

const MAX_ITEMS: [u32; 3] = [12, 13, 14];

fn possible_game_id(line: &str) -> u32 {
    let (id, actions) = decode_game(line);
    if id == 0 {
        return 0;
    }

    let bag = actions.fold([0, 0, 0], |acc, item| {
        let bag = calculate_bag_size(item);
        return [bag[0].max(acc[0]), bag[1].max(acc[1]), bag[2].max(acc[2])];
    });

    for i in 0..3 {
        if bag[i] > MAX_ITEMS[i] {
            return 0;
        }
    }

    return id;
}

fn decode_game(record: &str) -> (u32, str::Split<'_, &str>) {
    let parts = record.split(":").collect::<Vec<&str>>();
    if parts.len() != 2 {
        return (0, "".split(";"));
    }

    let id = parts[0]
        .trim()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    return (id, parts[1].split(";"));
}

fn decode_input(input: &str) -> (u32, &str) {
    let count = input.split(" ").nth(0).unwrap().parse::<u32>().unwrap();
    let color = input.split(" ").nth(1).unwrap();
    return (count, color);
}

fn calculate_bag_size(input: &str) -> [u32; 3] {
    let mut bag: [u32; 3] = [0, 0, 0];
    let input = input.split(",").into_iter();
    for item in input {
        let (count, color) = decode_input(item.trim());
        match color {
            "red" => bag[0] += count,
            "green" => bag[1] += count,
            "blue" => bag[2] += count,
            _ => (),
        }
    }
    return bag;
}
