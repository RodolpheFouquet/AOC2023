use nom::{
    bytes::complete::{tag},
    multi::many0,
    combinator::{map_res, recognize},
    branch::alt,
    character::complete::digit1,
    combinator::value,
    IResult,
    Parser,
};
use nom::character::complete::{line_ending, space0};
use nom::multi::{separated_list1, separated_list0};

const TEST: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

#[derive(Debug, PartialEq, Clone)]
enum SpringState {
    Damaged,
    Ok,
    Unknown
}

fn count_consecutive(v: &Vec<SpringState>) -> Vec<usize> {
    let mut consecutive = 0;
    let mut ret = Vec::new();
    v.iter().for_each(|a| {
        match a {
            SpringState::Damaged=> consecutive +=1,
            SpringState::Unknown | SpringState::Ok if consecutive > 0 => {
                ret.push(consecutive);
                consecutive = 0;
            }
            _ => ()
        }
    });

    if consecutive > 0 {
        ret.push(consecutive);
    }

    ret
}

fn parse_spring(input: &str) -> IResult<&str, SpringState> {
    alt((
        value(SpringState::Damaged, tag("#")),
        value(SpringState::Ok, tag(".")),
        value(SpringState::Unknown, tag("?")),
    ))(input)
}

fn is_spring(c: char) -> bool {
    match c {
        '.' | '#' | '?' => true,
        _ => false
    }
}

fn parse_springs(input: &str) -> IResult<&str, Vec<SpringState>> {
    many0(
        parse_spring,
    ).parse(input)
}

fn parse_number(input: &str) -> IResult<&str, u32> {
        map_res(recognize(digit1), str::parse)(input)
}

fn parse_consecutive(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list0(tag(","), parse_number)(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<SpringState>, Vec<u32>)> {
    println!("1", );
    let (remaining, springs) = parse_springs(input)?;
    let (remaining, _) = space0(remaining)?;
    let (remaining, consecutive) = parse_consecutive(remaining)?;

    Ok((remaining, (springs, consecutive)))
}


fn parse(input: &str) ->IResult<&str, Vec<(Vec<SpringState>, Vec<u32>)>> {
    separated_list1(line_ending, parse_line)(input)
}

fn main() {
    let input = TEST;

    let sp = parse(input);
    println!("{sp:?}");
}
