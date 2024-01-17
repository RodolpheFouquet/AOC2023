use std::ops::{Add, Sub};
use std::{cell::RefCell, rc::Rc};
use std::fmt::{Display, Formatter};
use std::cmp::Reverse;
use std::time::Instant;
use priority_queue::{PriorityQueue};
const TEST: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

impl Position {
    fn to_linear(&self, width: &usize) -> usize {
        self.x  as usize + (self.y as usize)*width
    }

    fn nil(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    fn to_arrow(&self) -> char {
        match (self.x, self.y) {
            (0, 1) => 'v',
            (0, -1) => '^',
            (1, 0) => '>',
            (-1, 0) => '<',
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Node {
    pos: Position,
    heat_loss: usize,
    neighbours: Vec<Rc<RefCell<Node>>>
}

impl Node {
    fn new(pos: Position, heat_loss: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            pos, heat_loss, neighbours: Vec::new()
        }))
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Node position: {}, heat loss: {}", self.pos, self.heat_loss)?;
        for n in &self.neighbours {
            writeln!(f, "\t- neighbour: {}, heat loss: {}", n.borrow().pos, n.borrow().heat_loss)?;
        }
        write!(f, "")
    }
}
struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
    width: usize
}

impl Graph {
    fn shortest_to(&self, start: usize, goal: usize) ->  Option<usize> {
        let mut distances = vec![usize::MAX; self.nodes.len()];
        let mut parents: Vec<usize> = (0..self.nodes.len()).collect::<Vec<_>>();
        distances[start] = 0;

        let mut queue = PriorityQueue::new();
        self.nodes.iter().enumerate().for_each(|(pos, node)| {
            queue.push(pos, Reverse(distances[pos]));
        });

        while let Some((current_pos, current_priority)) = queue.pop() {

            if current_pos == goal {
                break
            }
            for neighbour in &self.nodes[current_pos].borrow().neighbours {
                let neighbour_idx = neighbour.borrow().pos.to_linear(&self.width);
                let new_dist = distances[current_pos] + neighbour.borrow().heat_loss;
                //
                // if parents[current_pos] == neighbour_idx {
                //     continue;
                // }

                let mut p = current_pos.clone();
                let mut path = Vec::new();
                while parents[p] != p {
                    path.push(p);
                    p = parents[p];
                }


                let mut directions = path.windows(2).map(|window| self.nodes[window[0]].borrow().pos.clone() - self.nodes[window[1]].borrow().pos.clone()).collect::<Vec<_>>();
                if directions.len() >= 3 {
                    let new_direction = neighbour.borrow().pos.clone() - self.nodes[current_pos].borrow().pos.clone();
                    let three_last = vec![new_direction.clone(), directions[0].clone(), directions[1].clone(), directions[2].clone()];
                    if three_last.iter().all(|d| d.x == new_direction.x && d.y == new_direction.y) {
                        continue;
                    }
                }


                if new_dist < distances[neighbour_idx] {
                    distances[neighbour_idx] = new_dist;
                    parents[neighbour_idx] = current_pos;
                    queue.push_increase(neighbour_idx, Reverse(distances[neighbour_idx]));
                }
            }
        }



        println!("{}", distances.last().unwrap() );

        None
    }
}

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let heat_losses = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>())
        .collect::<Vec<_>>();

    let width = heat_losses[0].len();
    let height = heat_losses.len();
    let mut nodes = heat_losses.iter().enumerate().map(|(y, line)| {
        line.iter().enumerate().map(|(x, heat_loss)| {
            Node::new(Position{x: x as isize, y: y as isize}, *heat_loss)
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    for i in 0..width {
        for j in 0..height {
            let mut node = nodes[j][i].borrow_mut();

            if i > 0 {
                node.neighbours.push(nodes[j][i-1].clone());
            }
            if i < width-1 {
                node.neighbours.push(nodes[j][i+1].clone());
            }
            if j > 0 {
                node.neighbours.push(nodes[j-1][i].clone());
            }
            if j < width-1 {
                node.neighbours.push(nodes[j+1][i].clone());
            }
        }
    }

    let mut  g = Graph{nodes: nodes.iter().flat_map(|n| n.clone()).collect(), width};
    let dist  = g.shortest_to(0, width*height-1);

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);

}
