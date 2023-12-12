
const TEST: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

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

fn main() {
    let input = include_str!("../input.txt");
    //part 1
    let vertically_expanded_universe = input.lines().flat_map(|line| {
        if line.chars().all(|c| c == '.') {
            vec![line.chars().collect::<Vec<_>>(), line.chars().collect::<Vec<_>>()]
        } else {
            vec![line.chars().collect::<Vec<_>>()]
        }
    }).collect::<Vec<Vec<char>>>();
    let transposed_universe = transpose(vertically_expanded_universe.clone()).iter().flat_map(|line| {
        if line.iter().all(|c| *c == '.') {
            vec![line.clone(), line.clone()]
        } else {
            vec![line.clone()]
        }
    }).collect::<Vec<Vec<char>>>();

    let expanded_galaxy = transpose(transposed_universe.clone());

    let width = expanded_galaxy[0].len();
    let height = expanded_galaxy.len();

    let galaxies =
        expanded_galaxy
            .iter()
            .flat_map(|line| line)
            .enumerate()
            .filter(|(pos, &c)| c=='#')
            .map(|(pos, c)| ((pos%width) as i64, (pos/width) as i64)).collect::<Vec<_>>();

    let mut pairs = Vec::new();
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            pairs.push((galaxies[i] , galaxies[j]));
       }
    }

    let sum : i64= pairs.iter().map(|(g1, g2)| (g1.1-g2.1).abs()+(g1.0-g2.0).abs()).sum();
    println!("{}", sum);

    // part 2
    let galaxies = input.lines().collect::<Vec<&str>>();
    let sup_weight = 1000000;
    let width = galaxies[0].len();
    let height = galaxies.len();

    let empty_lines = galaxies
        .iter()
        .enumerate()
        .filter(|(pos, line)| line.chars().all(|c| c=='.'))
        .map(|(pos, line)| pos as i64).collect::<Vec<_>>();
    let empty_colums= transpose(vertically_expanded_universe.clone()).iter().enumerate().filter(|(pos, col)| col.iter().all(|c| *c=='.')).map(|(pos, col)| pos as i64).collect::<Vec<_>>();

    let galaxies =
        galaxies
            .iter()
            .flat_map(|line| line.chars().collect::<Vec<_>>())
            .enumerate()
            .filter(|(pos, c)| *c=='#')
            .map(|(pos, c)| ((pos%width) as i64, (pos/width) as i64)).collect::<Vec<_>>();


    let mut dist_pairs = Vec::new();
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let (min_x, max_x) = (galaxies[i].0.min(galaxies[j].0), galaxies[i].0.max(galaxies[j].0));
            let (min_y, max_y) = (galaxies[i].1.min(galaxies[j].1), galaxies[i].1.max(galaxies[j].1));


            let num_empty_lines = empty_lines.iter().filter(|&l| *l > min_y && *l < max_y).count() as i64;
            let num_empty_columns = empty_colums.iter().filter(|&c| *c > min_x && *c < max_x).count() as i64;
            let dist = (galaxies[i].1-galaxies[j].1).abs()+(galaxies[i].0-galaxies[j].0).abs() - num_empty_lines - num_empty_columns + sup_weight*(num_empty_lines + num_empty_columns);
            dist_pairs.push(dist);
        }
    }
    let sum :  i64= dist_pairs.iter().sum();

    println!("{:?}", sum);
}
