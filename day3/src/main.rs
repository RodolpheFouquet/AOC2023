use std::ops::{Add, AddAssign, Sub};

const TEST: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;
    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Position {
    fn adjacent(&self, other: &Self) -> bool {
        let neighbours = [
            *self + Position { x: -1, y: 0 },
            *self + Position { x: 1, y: 0 },
            *self + Position { x: 0, y: 1 },
            *self + Position { x: 0, y: -1 },
            *self + Position { x: 1, y: 1 },
            *self + Position { x: 1, y: -1 },
            *self + Position { x: -1, y: 1 },
            *self + Position { x: -1, y: -1 },
        ];
        neighbours.iter().any(|x| *x == *other)
    }
}

#[derive(Debug)]
struct Engine {
    width: usize,
    height: usize,
    map: Vec<char>,
    symbols: Vec<Position>,
    parts: Vec<i64>,
    stars: Vec<Position>,
}


impl From<&str> for Engine {
    fn from(map_string: &str) -> Self { 
        let width = map_string.lines().count(); 
        let height = map_string.lines().nth(0).unwrap().len(); 
        let mut stars = Vec::new(); 
        let mut all_nums: Vec<(i64, Vec<Position>)> = Vec::new();


        let map: Vec<char>= map_string.lines().flat_map(|l| l.chars().collect::<Vec<char>>()).collect();
        
        let symbols = map.iter().enumerate().map(|(pos, c)| {
            (c, Position{x: (pos % width) as i64, y: (pos / width) as i64})
        }).filter(|(&c, pos)| {
            if c == '*' {
                stars.push(pos.clone());
            }
            return c != '.' && !c.is_ascii_digit()
        }).map(|(c, pos)| pos).collect::<Vec<Position>>();

        let mut cur_num : Vec<(i64, char)> = Vec::new();
        let mut parts: Vec<i64> = Vec::new();
        for (pos, c) in map.iter().enumerate() {
            if c.is_ascii_digit() {
                cur_num.push((pos as i64, *c));
            } else if !cur_num.is_empty() {

                let first_pos = cur_num.first().unwrap().0;
                let last_pos = cur_num.last().unwrap().0;

                let first = Position{x: first_pos % width as i64, y: first_pos / width as i64};
                let last = Position{x: last_pos % width as i64, y: last_pos / width as i64};

                let adjacent_first = symbols.iter().any(|s| s.adjacent(&first));
                let adjacent_last = symbols.iter().any(|s| s.adjacent(&last));
                let number : i64 = cur_num.iter().map(|(pos, c)| c).collect::<String>().parse::<i64>().unwrap();
                if adjacent_first || adjacent_last {
                    parts.push(number);
                }

                let positions : Vec<Position> =  cur_num.iter()
                    .map(|(pos, c)| Position{x: pos % width as i64, y: pos / width as i64}).collect::<Vec<Position>>();
                all_nums.push((number,positions));
                cur_num.clear();
            }
        }
        let gears_ratios : i64 = stars.iter().map(|pos| {
            
            let adjacent_parts = all_nums.iter().filter(|(part, part_positions)| {
                pos.adjacent(part_positions.first().unwrap()) || pos.adjacent(part_positions.last().unwrap())
            }).map(|(part, position)| *part).collect::<Vec<i64>>();

            (pos.clone(), adjacent_parts)
        }).filter(|(pos, vec)| vec.len() == 2).map(|(p,v)| v[0]*v[1]).sum();
        println!("the sum of gear ratios is {}", gears_ratios); 
        Self{width, height, map, symbols, parts, stars}
    }
}



fn main() {

    let engine : Engine = include_str!("../input.txt").into();    
    println!("engine {}", engine.parts.iter().sum::<i64>());
}
