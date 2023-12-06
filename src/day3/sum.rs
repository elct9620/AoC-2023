pub fn part1(input: &str) -> u32 {
    let schematic = Schematic::new(input);

    schematic.iter().sum()
}

pub fn part2(_input: &str) -> u32 {
    0
}

struct Schematic {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Schematic {
    fn new(input: &str) -> Schematic {
        let data: Vec<Vec<char>> = input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();

        Schematic {
            data: data.clone(),
            width: data[0].len(),
            height: data.len(),
        }
    }

    fn is_symbol(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let c = self.data[y][x];
        match c {
            '.' => false,
            _ => !c.is_digit(10),
        }
    }

    fn take(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let c = self.data[y][x];
        if c.is_digit(10) {
            return Some(c);
        }

        None
    }

    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let mut adjacent = false;

        if x > 0 {
            adjacent = adjacent || self.is_symbol(x - 1, y) || self.is_symbol(x - 1, y + 1);
        }

        if y > 0 {
            adjacent = adjacent || self.is_symbol(x, y - 1) || self.is_symbol(x + 1, y - 1);
        }

        if x > 0 && y > 0 {
            adjacent = adjacent || self.is_symbol(x - 1, y - 1);
        }

        adjacent = adjacent
            || self.is_symbol(x + 1, y)
            || self.is_symbol(x, y + 1)
            || self.is_symbol(x + 1, y + 1);

        adjacent
    }

    fn iter(&self) -> SchematicIterator {
        SchematicIterator {
            schematic: self,
            x: 0,
            y: 0,
        }
    }
}

struct SchematicIterator<'a> {
    schematic: &'a Schematic,
    x: usize,
    y: usize,
}

impl<'a> Iterator for SchematicIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut adjacent = false;
        let mut digits: Vec<char> = Vec::new();

        loop {
            if self.x >= self.schematic.width {
                self.x = 0;
                self.y += 1;
            }

            if self.y >= self.schematic.height {
                return None;
            }

            let c = self.schematic.take(self.x, self.y);

            if c.is_none() {
                self.x += 1;
                break;
            }

            adjacent = adjacent || self.schematic.is_adjacent(self.x, self.y);
            digits.push(c.unwrap());
            self.x += 1;
        }

        let number = digits
            .iter()
            .collect::<String>()
            .parse::<u32>()
            .unwrap_or(0);

        if !adjacent || number == 0 {
            return self.next();
        }

        Some(number)
    }
}
