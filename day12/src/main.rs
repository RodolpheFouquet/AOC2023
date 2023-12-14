use std::collections::HashSet;
use std::fmt;
use std::ops::Sub;
use itertools::Itertools;
use std::iter;
use rayon::prelude::*;

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

impl From<char> for SpringState {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Damaged,
            '.' => Self::Ok,
            '?'=> Self::Unknown,
            _ => unreachable!()
        }
    }
}

impl fmt::Display for SpringState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        let c = match self {
            Self::Damaged => '#',
            Self::Ok => '.',
            Self::Unknown => '?',

        };
        write!(f, "{}", c)
    }
}

struct States(Vec<SpringState>);

impl fmt::Display for States {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().for_each(|c| {
            write!(f, "{}", c);
        });
        write!(f, "")
    }
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


fn main() {
    let input = include_str!("../input.txt");
    let records = input.lines().map(|line| {
        let comps = line.split_whitespace().collect::<Vec<&str>>();
        // part
        let r : String= [&comps[0].clone(), "?"].concat().repeat(5);
        let r =&r[0..r.len() - 1];
        let counts = comps.last().unwrap().split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
        (
            r.chars().map(|c| c.into()).collect::<Vec<SpringState>>(),
            counts.repeat(5)

        )
    }).collect::<Vec<_>>();

    let sum: u64 = records.par_iter().map(|(record, trace)| {
        let unknown_indices = record.iter().enumerate().filter(|&(pos, r)| *r == SpringState::Unknown).map(|(pos, r)| pos).collect::<HashSet<_>>();
        let s: States = States(record.clone());

        let mut score = 0;
        for i in 0..=unknown_indices.len() {
            for v  in unknown_indices.iter().combinations(i) {
                let v_as_hash = v.clone().into_iter().map(|n| *n).collect::<HashSet<_>>();
                let other = unknown_indices.sub(&v_as_hash);

                let mut rec_dmg = record.clone();
                for &a in &v {
                    rec_dmg[*a] = SpringState::Ok;
                    for b in &other {
                        rec_dmg[*b] = SpringState::Damaged;
                    }
                }
                if v.len() == 0 {
                    for b in &other {
                        rec_dmg[*b] = SpringState::Damaged;
                    }
                }
                let consecutive = crate::count_consecutive(&rec_dmg);
                if consecutive == *trace {
                    score +=1;
                }
            }
        }
        score
    }).sum();

    println!("{sum:?}");
}
