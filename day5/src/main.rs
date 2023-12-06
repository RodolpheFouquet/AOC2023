const TEST: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

#[derive(Debug)]
struct Range {
    source: u64,
    destination: u64,
    quantity: u64,
}

impl From<&str> for Range {
    fn from(v: &str) -> Self {
        let r = v.trim().split_whitespace().map(|n| n.parse::<u64>().unwrap()). collect::<Vec<u64>>();

        Self { source: r[1], destination: r[0], quantity: r[2]}
    } 
}

impl Range {
    fn map(&self, v: u64) -> Option<u64> {
    
        if v >= self.source && v <= self.source+self.quantity {
            let distance = v-self.source;
            Some(self.destination+distance)   
        } else {
            None
        }
    }
}

struct Map {
    seeds: Vec<u64>,
}


impl From<&str> for Map {
    fn from(s: &str) -> Self { 
        let split: Vec<&str> = s.trim().split("\n\n").collect(); 
        let seeds : Vec<u64> = split[0]
            .split(":").collect::<Vec<&str>>()[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap()).collect();
     

        let seeds_part2: Vec<(u64, u64)> = seeds
            .chunks(2)
            .map(|x| (x[0],x[0]+x[1])).collect();

        let names: Vec<String> = split[1..].iter().map(|c| c.lines().collect::<Vec<&str>>()[0].replace(":", "")).collect();
       let ranges: Vec<Vec<Range>> = split[1..]
           .iter()
           .map(|c|{

                c.lines()
                    .collect::<Vec<&str>>()[1..]
                    .iter()
                    .map(|&l| l.into())
                    .collect()
           }).collect();


        let min = seeds.iter().map(|s| {
            let mut seed = *s;
            let mut previous_seed = *s;
            ranges.iter().for_each(|ranges| {
                for r in ranges {
                    match r.map(seed) {
                        Some(s) => {
                            previous_seed = seed;
                            seed = s;
                            break;
                        },
                        None => ()
                    }
                }
            });
            seed

        }).min().unwrap();
       
        let mut min2 = -1 as i64;

        let mut i = 1;
        for (sd, sdf) in seeds_part2 {
            (sd..sdf).for_each(|soffset| {
                let mut seed = soffset;
                let mut previous_seed = seed;
                ranges.iter().for_each(|ranges| {
                    for r in ranges {
                        match r.map(seed) {
                            Some(s) => {
                                previous_seed = seed;
                                seed = s;
                                break;
                            },
                            None => ()
                        }
                    }
                });
                if min2 == -1 {
                    min2 = seed as i64;
                } else if (seed as i64) < min2 {
                    min2 = seed as i64;
                }
            } )

        }

        println!("{min}");
        println!("{min2}");

        Self {
            seeds: seeds, 
        }
    }
}

fn main() {
    let m : Map = include_str!("../input.txt").into();
    
}
