use std::collections::HashSet;
use std::ops::Sub;

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

fn find_reflection(pattern: &Pattern) -> Vec<(usize, usize)> {
    let potential_reflections = pattern.iter().enumerate().zip(pattern.iter().enumerate().skip(1)).filter(|&(a, b)| {
        a.1==b.1
    }).map(|(a, b)| (a.0, b.0)).collect::<Vec<_>>();

    potential_reflections.iter().filter(|(a, b)| {
        let mut has_reflection = true;
        let binding = (pattern.len()-b);
        let max_size = (a+1usize).min(binding);
        // println!("max {a} {b} {max_size} {}", max_size);
        for i in (1..max_size) {
            if pattern[a-i] != pattern[b+i] {
                has_reflection = false;
            }
            // println!("loop {i} {max_size} {has_reflection}");
        }
        has_reflection
    }).map(|(a, b)| (*a, *b)).collect::<Vec<_>>()
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

    // part 1
    let total: usize = patterns.iter().map(|pattern| {
        let reflection = find_reflection(pattern);
        if let Some(r) = reflection.first() {
            return (r.0+1)*100;
        }

        let transposed_pattern = transpose(pattern.clone());
        let reflection = find_reflection(&transposed_pattern);
        if let Some(r) = reflection.first()  {
            return (r.0+1);
        }
        return 0;
    }).sum();
    println!("{total}");

    // part 2
    // I could do with much less copies
    // but heck
    let total2: usize = patterns.iter().map(|pattern| {
        let before_cloned = pattern.clone();
        let before_transposed = transpose(before_cloned);
        let (before_rh, before_rv) = (find_reflection(pattern), find_reflection(&before_transposed));
        let (before_rh, before_rv) = (before_rh.iter().collect::<HashSet<_>>(), before_rv.iter().collect::<HashSet<_>>());
        for i in 0..pattern.len() {
            for j in 0..pattern[i].len() {
                let mut p = pattern.clone();
                p[i][j] = match p[i][j] {
                    '.' => '#',
                    '#' => '.',
                    _ => unreachable!()
                };
                let cloned = p.clone();
                let transposed = transpose(cloned);
                let (rh, rv) = (find_reflection(&p), find_reflection(&transposed));
                let (rh, rv) = (rh.iter().collect::<HashSet<_>>(), rv.iter().collect::<HashSet<_>>());

                if let Some(r) = rh.sub(&before_rh).iter().next() {
                    return (r.0+1)*100;
                }
                if let Some(r) = rv.sub(&before_rv).iter().next()   {
                    return (r.0+1);
                }
            }
        }

        return 0;
    }).sum();
    println!("{total2}");
}
