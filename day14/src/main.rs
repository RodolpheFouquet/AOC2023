use std::collections::{HashMap, HashSet};
use std::{fmt, vec};
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

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
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

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn reverse_lines<T>(v: &mut Vec<Vec<T>>)  {
    v.iter_mut().for_each(|line| {
        line.reverse()
    })
}

fn rotate90deg<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>{
    let mut r = transpose(v);
    reverse_lines(&mut r);
    r
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

    fn rotate(&mut self) {
        self.rocks = rotate90deg(self.rocks.clone());
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

    let total_steps = 1000000000usize;
    let mut past_dishes = HashMap::new();
    let mut dishes: Vec<Vec<Vec<RockType>>> = Vec::new();
    let mut cycle_len = 0usize;
    let mut cycle_start = 0usize;
    for i in 0..total_steps {

        for j in 0..4 {
            dish.tilt();
            dish.rotate();
        }
        if let Some(r) = past_dishes.get(&dish.rocks.clone()) {
            if cycle_start == 0 {
                cycle_start = i;
            } else if cycle_len == 0 && dish.rocks == dishes[cycle_start] {
                cycle_len = i - cycle_start;
            } else if cycle_start != 0 && cycle_len != 0 {
                let modulo = (i - cycle_start) % cycle_len+cycle_start;
                let modulo = (1000usize-1 - cycle_start) % cycle_len+cycle_start;
                dish.rocks = dishes[modulo].clone();
                let load = dish.load();
                println!("{load}");
                return
            }
        }

        dishes.push(dish.rocks.clone());
        past_dishes.entry(dish.rocks.clone()).or_insert(i*4+i);
    }

}
