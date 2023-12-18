use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, Formatter};
use nom::{
    bytes::complete::{tag},
    combinator::{map_res, recognize},
    branch::alt,
    character::complete::digit1,
    combinator::value,
    IResult,
    Parser,
};
use std::ops::Add;
use nom::character::complete::line_ending;
use nom::multi::{many0, separated_list1};


const TEST : &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

#[derive(Debug, Clone, PartialEq)]
enum TileType{
    Empty, // .
    HorizontalSplitter, // -
    VerticalSplitter, // |
    ForwardMirror, // /
    BackMirror // \
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: i64,
    y: i64,
}
type Vector = Point;

#[derive(Debug, Clone)]
struct Tile {
    position: Point,
    tile_type: TileType,
    energized: HashSet<Vector>
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.x == 1 && self.y == 0 {
            write!(f, ">" )
        } else {
            write!(f, "" )
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.energized.len()> 0 {
            write!(f, "#")
        } else {
            let c = match self.tile_type {
                TileType::Empty => ".",
                TileType::HorizontalSplitter => "-",
                TileType::VerticalSplitter => "|",
                TileType::ForwardMirror => "/",
                TileType::BackMirror => "\\",
            };
            write!(f, "{}", c)
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn parse_tile(input: &str) -> IResult<&str, TileType> {
    alt((
        value(TileType::Empty, tag(".")),
        value(TileType::HorizontalSplitter, tag("-")),
        value(TileType::VerticalSplitter, tag("|")),
        value(TileType::ForwardMirror, tag("/")),
        value(TileType::BackMirror, tag("\\")),
    ))(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<TileType>> {
    many0(
        parse_tile,
    ).parse(input)
}

#[derive(Debug, Clone)]
struct Contraption{
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    in_flight_beams: Vec<(Point,Vector)>
}

impl Display for Contraption {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for i in 0..self.width {
            for j in 0..self.height {
                write!(f, "{}", self.tiles[i*self.width+j])?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")
    }
}

impl Contraption {
    fn parse(input: &str) -> Vec<Vec<TileType>> {
        let (remaining, tiles)= separated_list1(line_ending, parse_line)(input).unwrap();
        assert!(remaining.is_empty());

        tiles
    }

    fn shine(&mut self, start: Point, direction: Vector) {
        if self.in_flight_beams.is_empty() {
            self.in_flight_beams.push((start, direction))
        }

        while !self.in_flight_beams.is_empty() {
            self.process_beam();
        }
    }

    fn process_beam(&mut self) {
        let (position, direction) = self.in_flight_beams.pop().unwrap();

        if position.x < 0 || position.y < 0
            || position.x >= self.width as i64 || position.y >= self.height as i64 {
            // beam out of bounds
            return
        }

        let mut tile = self.get_mut(position);
        let tile_pos = tile.position.clone();
        // already energized from the same direction, we don't need to proceed anymore
        if tile.energized.contains(&direction) {
            return
        }
        tile.energized.insert(direction.clone());

        match tile.tile_type {
            TileType::Empty => {
                self.in_flight_beams.push((tile_pos+direction.clone(), direction.clone()));
            },
            TileType::HorizontalSplitter => {
                if direction.x == 1 || direction.x == -1 {
                    self.in_flight_beams.push((tile_pos+direction.clone(), direction.clone()));
                } else {
                    self.in_flight_beams.push((tile_pos.clone()+Vector{x: 1, y: 0}, Vector{x: 1, y: 0}));
                    self.in_flight_beams.push((tile_pos+Vector{x: -1, y: 0}, Vector{x: -1, y: 0}));
                }
            },
            TileType::VerticalSplitter => {
                if direction.y == 1 || direction.y == -1 {
                    self.in_flight_beams.push((tile_pos+direction.clone(), direction.clone()));
                } else {
                    self.in_flight_beams.push((tile_pos.clone()+Vector{x:0 , y: 1}, Vector{x: 0, y: 1}));
                    self.in_flight_beams.push((tile_pos+Vector{x: 0, y: -1}, Vector{x: 0, y: -1}));
                }
            },
            TileType::ForwardMirror => {
                if direction.x == 1 {
                    self.in_flight_beams.push((tile_pos+Vector{x: 0, y: -1}, Vector{x: 0, y: -1}));
                } else if  direction.x == -1{
                    self.in_flight_beams.push((tile_pos.clone()+Vector{x: 0, y: 1}, Vector{x: 0, y: 1}));
                } else if  direction.y == 1 {
                    self.in_flight_beams.push((tile_pos.clone()+Vector{x: -1, y: 0}, Vector{x: -1, y: 0}));
                } else if  direction.y == -1{
                    self.in_flight_beams.push((tile_pos.clone()+Vector{x: 1, y: 0}, Vector{x: 1, y: 0}));
                }
            },
            TileType::BackMirror => {
                if direction.x == 1 {
                    self.in_flight_beams.push((tile_pos+Vector{x: 0, y: 1}, Vector{x: 0, y: 1}));
                } else if  direction.x == -1{
                    self.in_flight_beams.push((tile_pos.clone()+Vector{x: 0, y: -1}, Vector{x: 0, y: -1}));
                } else if  direction.y == 1 {
                    self.in_flight_beams.push((tile_pos.clone()+Vector{x: 1, y: 0}, Vector{x: 1, y: 0}));
                } else if  direction.y == -1{
                    self.in_flight_beams.push((tile_pos.clone()+Vector{x: -1, y: 0}, Vector{x: -1, y: 0}));
                }
            },
            _ => unreachable!()
        }
    }

    fn count_energized(&self)-> usize {
        self.tiles.iter().filter(|tile| tile.energized.len() >0).count()
    }

    fn get(&self, pos: Point) -> &Tile {
        let p = pos.y*(self.width as i64)+pos.x;
        &self.tiles[p as usize]
    }

    fn get_mut(&mut self, pos: Point) -> &mut Tile {
        let p = pos.y*(self.width as i64)+pos.x;
        &mut self.tiles[p as usize]
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let tile_types = Contraption::parse(input);
    let width = tile_types[0].len();
    let height = tile_types.len();

    let mut tiles = tile_types.iter().enumerate().flat_map(|(i, line)| {
        line.iter().enumerate().map(|(j, &ref tile_type)| {
            Tile {
                energized: HashSet::new(),
                tile_type: tile_type.clone(),
                position: Point {
                    x: j as i64,
                    y: i as i64
                }
            }
        }).collect::<Vec<Tile>>()
    }).collect::<Vec<Tile>>();
    let mut contraption = Contraption{tiles: tiles, width, height, in_flight_beams: Vec::new()};

    // let mut s = 0;
    let mut starting_config = Vec::new();
    for i in 0..width {
        starting_config.push((Point{x: i as i64, y: 0}, Vector{x: 0, y: 1}));
        starting_config.push((Point{x: i as i64, y: (height -1) as i64}, Vector{x: 0, y: -1}));
    }

    for j in 0..height {
        starting_config.push((Point{x: 0, y: j as i64}, Vector{x: 1, y: 0}));
        starting_config.push((Point{x: (width -1) as i64, y: j as i64}, Vector{x: -1, y: 0}));
    }

    let max: usize = starting_config.iter().map(|(start, direction)| {
        let mut c = contraption.clone();
        c.shine(start.clone(), direction.clone());
        c.count_energized()
    }).max().unwrap();
    println!("{}", max)
 }
