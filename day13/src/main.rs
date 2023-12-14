const TEST: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

#[derive(Debug)]
enum Symmetry {
    Vertical,
    Horizontal
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

type Pattern = Vec<Vec<char>>;
fn main() {
    let input = include_str!("../input.txt");
    let patterns = input.split("\n\n")
        .map(|pattern| {
            pattern.lines().map(|line| {
                line.chars().collect::<Vec<char>>()
            }).collect::<Pattern>()
        })
        .collect::<Vec<Pattern>>();


    let total: usize = patterns.iter().map(|pattern| {
        let potential_reflections = pattern.iter().enumerate().zip(pattern.iter().enumerate().skip(1)).filter(|&(a, b)| {
        a.1==b.1
        }).map(|(a, b)| (a.0, b.0)).collect::<Vec<_>>();


        let reflection = potential_reflections.iter().filter(|(a, b)| {
            let mut has_reflection = true;
            let binding = (pattern.len()-b);
            let max_size = (a+1usize).min(binding);
            for i in (1..max_size) {
                if pattern[a-i] != pattern[b+i] {
                    has_reflection = false;
                }
            }
            has_reflection
        }).collect::<Vec<_>>();

        if let Some(&r) = reflection.first() {
            return (r.0+1)*100;
        }

        let transposed_pattern = transpose(pattern.clone());
        let potential_reflections = transposed_pattern.iter().enumerate().zip(transposed_pattern.iter().enumerate().skip(1)).filter(|&(a, b)| {
            a.1==b.1
        }).map(|(a, b)| (a.0, b.0)).collect::<Vec<_>>();

        let reflection = potential_reflections.iter().filter(|(a, b)| {
            let mut has_reflection = true;
            let binding = (transposed_pattern.len()-b);
            let max_size = (a+1usize).min(binding);
            for i in (1..max_size) {
                if transposed_pattern[a-i] != transposed_pattern[b+i] {
                    has_reflection = false;
                }
            }
            has_reflection
        }).collect::<Vec<_>>();

        if let Some(&r) = reflection.first() {
            return (r.0+1);
        }
        return 0;

    }).sum();
    println!("{total}")
}
