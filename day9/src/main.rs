
const TEST: &str  = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;


const TEST2: &str = r#"10  13  16  21  30  45"#;
fn extrapolate(input: &Vec<i64>) -> i64 {
    let mut vecs: Vec<Vec<i64>> = Vec::new();
    vecs.push(input.clone());
    loop {
        let line : Vec<_>= vecs.last().unwrap().windows(2).map(|window| {
            window[1] - window[0]
        }).collect();
        vecs.push(line.clone());
        if line.iter().all(|&x| x == 0) {
            break;
        }
    }

    let res: i64 = vecs.iter().rev().map(|x| x.last().unwrap()).sum();
    res
}

fn extrapolate_back(input: &Vec<i64>) -> i64 {
    let mut vecs: Vec<Vec<i64>> = Vec::new();
    vecs.push(input.clone());
    loop {
        let line : Vec<_>= vecs.last().unwrap().windows(2).map(|window| {
            window[1] - window[0]
        }).collect();
        vecs.push(line.clone());
        if line.iter().all(|&x| x == 0) {
            break;
        }
    }
    vecs.iter_mut().for_each(|v| v.reverse());

    for i in 1..vecs.len() {
        let new_val = vecs[vecs.len()-i-1].last().unwrap()-vecs[vecs.len()-i].last().unwrap();
        let x = vecs.len()-i-1;
        vecs[x].push(new_val);
    }

    *vecs[0].last().unwrap()
}

fn main() {
    let input = include_str!("../input.txt");
    let values : Vec<_>= input.lines().map(|line| line.split_whitespace().map(|i| i.parse::<i64>().unwrap()).collect::<Vec<i64>>()).collect();


    //let sum: i64= values.iter().map(|v| extrapolate(&v)).sum();
    let sum: i64= values.iter().map(|v| extrapolate_back(&v)).sum();
    println!("{sum:?}");
}
