use std::collections::{HashSet, VecDeque};

const TEST : &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

#[derive(Debug, Clone)]
struct Card {
    id: i64,
    winning_numbers: HashSet<i64>,
    my_numbers: HashSet<i64>
}


impl From<&str> for Card {
    fn from(line: &str) -> Self {
        let components = line.split(':').collect::<Vec<&str>>();
        let id = components[0]
            .split_whitespace().collect::<Vec<&str>>()[1].parse::<i64>().unwrap();
        
        let numbers = components[1].split('|').collect::<Vec<&str>>();
        let winning_numbers = numbers[1].trim().split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
        let my_numbers = numbers[0].trim().split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();

        Self {
            id, winning_numbers, my_numbers
        }
    }
}


impl Card {

    fn score(&self) -> i64 {
        let number_of_wins = self.my_numbers.intersection(&self.winning_numbers).count() as u32;
        if number_of_wins == 0 {
            0
        } else {
            i64::pow(2, number_of_wins - 1)
        }
    }

    fn score_part2(&self) -> i64 {
        self.my_numbers.intersection(&self.winning_numbers).count() as i64
    }
}



fn main() {

    let cards : Vec<Card> = include_str!("../input.txt").lines().map(|l| Card::from(l)).collect();
    let score : i64 = cards.iter().map(|c| c.score()).sum();
    
    let mut card_queue : VecDeque<Card> = VecDeque::from(cards.clone()); 
    let mut count = 0;

    while let Some(card) = card_queue.pop_front() {
        count+= 1;
        let score = card.score_part2();
        cards[(card.id) as usize..(card.id+score) as usize].iter().for_each(|c| {
            card_queue.push_back(c.clone());
        })
    }

    println!("{:?}", score);
    println!("{:?}", count);
}
