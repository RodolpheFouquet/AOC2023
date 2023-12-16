use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::{tag},
    combinator::{map_res, recognize},
    branch::alt,
    character::complete::digit1,
    combinator::value,
    IResult,
    Parser,
};
use nom::bytes::complete::{take_till, take_while};
use nom::multi::separated_list0;

const TEST: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

fn hash(v: &str) -> usize {
    v.chars().map(|c| c as usize).fold(0, |acc, c| {
        let r = ((acc + c)*17)%256;
        r
    })
}

#[derive(Debug, Clone)]
enum Operation {
    Remove,
    Add
}

fn op(input: &str) -> IResult<&str, Operation> {
    alt((
        value(Operation::Add, tag("=")),
        value(Operation::Remove, tag("-")),
    ))(input)
}

fn is_ascii(c: char) -> bool {
    c.is_ascii_alphabetic()
}


fn from_label(input: &str) -> Result<&str, ()> {
    Ok(input)
}
fn label(input: &str) -> IResult<&str, &str> {
    map_res(
        take_while(is_ascii),
        from_label
    ).parse(input)
}

fn focal(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_item(input: &str) -> IResult<&str, (&str, Operation, usize)> {
    let (remaining, parsed_label) = label(input)?;
    let (remaining, operator) = op(remaining)?;
    let (remaining, num) = match operator {
        Operation::Add => focal(remaining)?,
        Operation::Remove=> (remaining, 0),
    };

    Ok((remaining, (parsed_label, operator, num)))
}

fn parse(input: &str) -> IResult<&str, Vec<(&str, Operation, usize)>> {
    separated_list0(tag(","), parse_item)(input)
}

fn main() {
    let input = include_str!("../input.txt");
    // part 1
    let s: usize = input.split(',').map(|s| hash(s)).sum();
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];

    let lenses = parse(input).unwrap().1;
    lenses.iter().for_each(|(label, operation, focal_length)| {
        let b = hash(label);

        match operation {
            Operation::Add => {
                let p = boxes[b as usize].iter().position(|c| c.0 == *label);
                match p {
                    Some(p)=> boxes[b as usize][p]=(label, *focal_length),
                    None => boxes[b as usize].push((label, *focal_length))
                };

            },
            Operation::Remove => {
                let p = boxes[b as usize].iter().position(|c| c.0 == *label);
                if let Some(pos) = p {
                    boxes[b as usize].remove(pos);
                }
            }
        };

    });


    let score: usize = boxes.iter().enumerate().map(|(box_num, b)| b.iter().enumerate().map(|(pos, lens)| {
        (box_num+1)*(pos+1)*lens.1
    }).sum::<usize>()).sum();
    println!("{score}");
}
