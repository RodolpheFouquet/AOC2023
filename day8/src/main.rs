use std::collections::HashMap;
use num::Integer;

const TEST: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;


const TEST2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

const TEST3: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

#[derive(Debug)]
enum Instruction {
    Left,
    Right
}

fn main() {

    let input =  include_str!("../input.txt");
    let comps = input.split("\n\n").collect::<Vec<&str>>();

    let instructions = comps[0].chars().map(|c| match c {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _   => unreachable!()
    }).collect::<Vec<Instruction>>();


    let mut starting_nodes: Vec<String> = Vec::new();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    comps[1].lines().for_each(|l| {

        let comps = l.split(" = ").collect::<Vec<&str>>();

        let node = comps[0];

        if node.ends_with("A") {
            starting_nodes.push(node.to_string());
        }

        let pair = comps[1].split(", ").collect::<Vec<&str>>();

        let left = pair[0].replace("(", "");
        let right = pair[1].replace(")", "");
    
        nodes.insert(node.to_string(), (left, right));
    });

    let mut i: usize = 0;
    let mut steps: usize = 0;
    // part 1
 /*   let mut current = "AAA".to_string();
    while current != "ZZZ" {
        let instruction = &instructions[i];

        let next = nodes.get(&current).unwrap();
        current = match instruction {
            Instruction::Left => next.0.clone(),
            Instruction::Right => next.1.clone()
        };

        steps +=1;
        i = steps%instructions.len();
    }
    println!("steps for part 1 {steps}");
*/
    println!("starting {starting_nodes:?}");
    

    //part 2
    let mut node_steps = vec![0; starting_nodes.len()];
    let node_steps = starting_nodes.iter().map(|n| {
        let mut cur_node = n.clone();
        let mut steps = 0;
        while !cur_node.ends_with("Z") {
            let instruction = &instructions[steps%instructions.len()];

            let next = nodes.get(&cur_node).unwrap();
            cur_node = match instruction {
                Instruction::Left => next.0.clone(),
                Instruction::Right => next.1.clone()
            };

            steps +=1;
        }
        steps
    }).collect::<Vec<usize>>().iter().fold(1,|acc, num| Integer::lcm(&acc, num));

    println!("steps for part 2 {node_steps:?}");
}
