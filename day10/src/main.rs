use std::collections::HashSet;

const TEST: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

const TEST2: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;


fn char_to_pos(c: &char, starting_post: &mut usize, pos: &usize, width: &usize, height: &usize) -> (char , Vec<usize>) {
    let vec_east = if (pos+1)%width != 0 {
        vec![pos+1]
    } else {
        vec![]
    };
    let vec_west = if pos%width != 0 {
        vec![pos-1]
    } else {
        vec![]
    };
    let vec_north = if pos>=width {
        vec![pos-width]
    } else {
        vec![]
    };
    let vec_south = if pos/width < height-1  {
        vec![pos+width]
    } else {
        vec![]
    };

    (
        *c,
        match c {
            'S' => {
                *starting_post = *pos;
                [vec_east, vec_west, vec_north, vec_south].concat()
            },
            '|' => {
                [ vec_north, vec_south].concat()
            },
            '-' => {
                [vec_west, vec_east].concat()
            },
            'L' => {
                [vec_east, vec_north].concat()
            },
            'J' => {
                [vec_west, vec_north].concat()
            },
            '7' => {
                [vec_south, vec_west].concat()
            },
            'F' => {
                [vec_south,  vec_east].concat()
            },
            '.' => {
                Vec::new()
            },
            _ => unreachable!()
        }
    )
}


type Node = Vec<(char, Vec<usize>)>;

#[derive(Debug)]
struct PipeGraph {
    starting_position: usize,
    nodes: Node
}

impl PipeGraph{
    fn cycles(&self) -> Vec<usize> {
        let mut visited = vec![0; self.nodes.len()];
        let mut ret = Vec::new();
        // self.dfs(self.starting_position, self.starting_position, 0, &mut visited, &mut cycles );

        let mut stack = Vec::new();
        stack.push((self.starting_position, self.starting_position, Vec::new()));
        while !stack.is_empty() {
            let (pos, prev, cycle) = stack.pop().unwrap();
                visited[pos] += 1;
                if visited[pos] == 2 && cycle.len() > 1 {
                    ret = cycle;
                    break
                } else if visited[pos] != 2 {
                    for node in self.nodes[pos].1.iter() {
                        if *node==prev {
                            // println!("node {node} is the parent, do not visit");
                        } else {
                            let mut c = cycle.clone();
                            c.push(pos);
                            stack.push((*node, pos, c));
                        }
                    }
                }
        }

        ret
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut starting_position = 0usize;
    let width = input.lines().collect::<Vec<&str>>()[0].len();
    let height = input.lines().collect::<Vec<&str>>().len();
    let map = input.lines().flat_map(|l| {
        l.chars().collect::<Vec<char>>()
    }).enumerate().map(|(pos, c)| {
        char_to_pos(&c, &mut starting_position, &pos, &width, &height)
    }).collect::<Vec<_>>();

    let nodes = map.iter().enumerate().map(|(pos, node)| {
        (
            node.0,
            node.1.iter().filter(  |i| {
                map[**i].1.iter().position(|x| pos==*x).is_some()
            }).map(|x| *x).collect::<Vec<_>>()
        )
    }).collect::<Vec<_>>();

    let graph = PipeGraph{nodes: nodes.clone(), starting_position};

    let res = graph.cycles();
    let visited = res.iter().collect::<HashSet<_>>();
    // println!("{res:?}");
    let max_dist = res.len()/2;
    // println!("{max_dist:?}");

    let mut count_inside = 0;
    for (pos, node) in nodes.iter().enumerate() {
        let x = pos%width;
        if visited.contains(&pos) {
            continue
        }

        let mut count_crossings = 0;
        for i in (1..=x).rev() {
            let p = pos-i;
            let c =  nodes[p].0;
            if !visited.contains(&p) {
                continue
            }
            if c == 'J' || c == 'L' || c == '|' {
                count_crossings += 1;
            }

        }
        if count_crossings%2 == 1 {
            count_inside+=1;
        }

    }

    println!("{count_inside} are inside the loop");
}
