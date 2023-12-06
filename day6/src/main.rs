const TEST : &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

fn main() {

    // part 1
    {
    let times = include_str!("../input.txt")
        .lines()
        .collect::<Vec<&str>>()[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    
    let distances = include_str!("../input.txt")
        .lines()
        .collect::<Vec<&str>>()[1]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    
    let times_and_distances : Vec<(i64, i64)> = times
        .into_iter()
        .zip(distances.into_iter())
        .collect();
    
    let res = times_and_distances.iter().map(|(t,d)| {
        let a = (1 as i64..*t).map(|tt| *d+tt*tt-tt*t).filter(|&r| r < 0).count();
        println!("{a}");
        a

    }).reduce(|a,b| a*b).unwrap(); 
    println!("{times_and_distances:?}");
    println!("{res:?}");
   }
     // part 2
    {
    let time = include_str!("../input.txt")
        .lines()
        .collect::<Vec<&str>>()[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .chars().filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>().unwrap();
    
    let distance = include_str!("../input.txt") 
        .lines()
        .collect::<Vec<&str>>()[1]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .chars().filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>().unwrap();
    
    let possibilities = (1 as i64..time).map(|tt| distance+tt*tt-tt*time).filter(|&r| r < 0).count();
    println!("{time:?}");
    println!("{distance:?}");
    println!("{possibilities:?}");
   }   
}
