use std::fmt;
use std::fmt::Formatter;

const TEST: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

#[derive(Debug, PartialEq)]
enum RockType {
    Solid,
    Rounded,
    Empty
}

impl From<char> for RockType {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Solid,
            'O' => Self::Rounded,
            _ => unreachable!()
        }
    }
}

struct Dish {
    rocks: Vec<Vec<RockType>>
}

impl Dish {
    fn tilt(&mut self) {
        let columns = self.rocks[0].len();
        let height = self.rocks.len();
        for i in 0..columns {
            let mut position_round = Vec::new();
            let mut current_spot = 0usize;
            for j in 0..height {
                match self.rocks[j][i] {
                    RockType::Empty => (),
                    RockType::Rounded => {
                        position_round.push(j);
                    },
                    RockType::Solid => {
                        position_round.iter().for_each(|p| {
                            self.rocks[*p][i] =  RockType::Empty;
                        });
                        for x in current_spot..current_spot+position_round.len() {
                            self.rocks[x][i] =  RockType::Rounded;
                        }
                        position_round.clear();
                        current_spot = j+1;
                    }
                }
            }
            if !position_round.is_empty() {
                position_round.iter().for_each(|p| {
                    self.rocks[*p][i] =  RockType::Empty;
                });
                for x in current_spot..current_spot+position_round.len() {
                    self.rocks[x][i] =  RockType::Rounded;
                }
                position_round.clear();
            }
        }
    }

    fn load(&self) -> usize {
        let max_weight = self.rocks.len();

        self.rocks.iter().enumerate().map(|(pos, line)| {
            line.iter().filter(|&r| *r == RockType::Rounded).count() * (max_weight - pos)
        }).sum()
    }
}

impl fmt::Display for Dish {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.rocks.iter().for_each(|rock_line| {
           rock_line.iter().for_each(|r| {
               let c = match r {
                   RockType::Empty => '.',
                   RockType::Rounded => '0',
                   RockType::Solid => '#',
               };
               write!(f, "{c}");
           });
            writeln!(f, "");
        });
        write!(f, "")
    }
}


fn main() {

    let input = include_str!("../input.txt");

    let rocks: Vec<Vec<RockType>> = input.lines().map(|line| line.chars().map(|c| c.into()).collect()).collect();
    let mut dish = Dish{rocks};
    println!("{dish}");
    dish.tilt();;
    println!("{dish}");
    let load = dish.load();
    println!("{load}");
}
