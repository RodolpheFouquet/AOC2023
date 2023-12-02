use std::cmp::max;

const TEST: &str= r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

#[derive(Debug, PartialEq)]
enum Colour {
    Red(i64),
    Green(i64),
    Blue(i64)
}

impl TryFrom<&str> for Colour {
    type Error = String;
    fn try_from(v: &str) -> Result<Self, Self::Error> { 
        let components: Vec<&str> = v.trim().split(' ').collect();
        if components.len() != 2 {
            return Err(String::from("invalid number of compenents to parse move"));
        }

        let number = components.first().unwrap().parse::<i64>().map_err(|e| String::from("unable to parse int for move"))?;

        match *components.last().unwrap() {
            "blue" => Ok(Colour::Blue(number)),
            "red" => Ok(Colour::Red(number)),
            "green" => Ok(Colour::Green(number)),
            _ => Err(String::from("invalid move")),
        } 
    }
}

struct Turn {
    colours: Vec<Colour>
}

impl Turn {
    fn is_possible(&self, red: i64, green: i64, blue: i64) -> bool { 
        self.colours.iter().map(|c| match c {
            Colour::Red(i) => *i <= red,
            Colour::Green(i) => *i <= green,
            Colour::Blue(i) => *i <= blue,
        }).all(|cond| cond)
    }
    
    fn needed_for_turn(&self) -> (i64, i64, i64) {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        self.colours.iter().for_each(|c| {
            match c {
                Colour::Red(i) if *i > min_red => min_red = *i ,
                Colour::Green(i) if *i > min_green => min_green = *i,
                Colour::Blue(i) if *i > min_blue => min_blue = *i,
                _ => (),
            }
        });

        (min_red, min_green, min_blue)
    }
}


impl TryFrom<&str> for Turn { 
    type Error = String;

    fn try_from(v: &str) -> Result<Self, Self::Error> { 
        let cls: Result<Vec<Colour>, Self::Error> = v.trim().split(',')
                                                            .map(|c| c.try_into()).collect();

        match cls {
            Ok(colours) => Ok(Turn{colours}),
            Err(e) => Err(e)
        }

    }
}

struct Game {
    id: i64,
    turns: Vec<Turn>
}

impl TryFrom<&str> for Game {
    type Error = String;
    fn try_from(v: &str) -> Result<Self, Self::Error> { 
        let components : Vec<&str> = v.trim().split(":").collect();
        if components.len() != 2 {
            return Err(String::from("invalid number of compenents to parse game"));
        }
        let id = components.first().unwrap().replace("Game ", "").parse::<i64>().map_err(|e| String::from("invalid game id"))?;

        let t : Result<Vec<Turn>, Self::Error> = components.last()
                        .unwrap()
                        .split(';')
                        .map(|m| Turn::try_from(m)).collect();

        match t {
            Ok(turns) => Ok(Game{ id, turns}),
            Err(e) => Err(e)
        }
        
    }
}

impl Game {
    fn is_possible(&self, red: i64, green: i64, blue: i64) -> bool { 
        self.turns.iter().map(|t| t.is_possible(red, green, blue)).all(|cond| cond)
    }

    fn required(&self) -> (i64, i64, i64) {
        self.turns.iter()
            .map(|t| t.needed_for_turn())
            .fold((0, 0, 0), |acc, x| {
                (max(acc.0, x.0), max(acc.1, x.1), max(acc.2, x.2))
            })
    }
}

fn main() {
    let sum : i64 = include_str!("../input.txt").lines()
            .map(|l| Game::try_from(l).unwrap())
            .filter(|g| g.is_possible(12, 13, 14))
            .map(|g| g.id)
            .sum();


    let power: i64 = include_str!("../input.txt").lines()
            .map(|l| Game::try_from(l).unwrap())
            .map(|g| g.required())
            .map(|(r, g, b)| r*g*b)
            .sum();
            
    println!("the sum is {}", sum);
    println!("the power is {}", power);
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_try_from_colour() {
        assert_eq!(Colour::Blue(1), "1 blue".try_into().unwrap());
        assert_eq!(Colour::Red(3), "3 red".try_into().unwrap());
        assert_eq!(Colour::Green(5), "5 green".try_into().unwrap());
        

        assert_eq!(true, Colour::try_from("x green").is_err());
        assert_eq!(true, Colour::try_from("1 adsd").is_err());
        assert_eq!(true, Colour::try_from("1 ").is_err());
    }

    #[test]
    fn test_try_turn() {
        let turn: Result<Turn, String> = " 1 red, 2 green".try_into();
    
        assert_eq!(true, turn.is_ok());
        assert_eq!(Colour::Red(1), turn.as_ref().unwrap().colours[0]);
        assert_eq!(Colour::Green(2), turn.as_ref().unwrap().colours[1]);

        assert_eq!(true, Turn::try_from(" asd blue, asdasd red").is_err());
        
    }


    #[test]
    fn test_is_possible_turn() {
        let turn: Turn = " 10 red, 2 green".try_into().unwrap();
        assert_eq!(true, turn.is_possible(10, 2, 1)); 
        assert_eq!(false, turn.is_possible(5, 2, 1)); 
        assert_eq!(true, turn.is_possible(11, 2, 1)); 
        assert_eq!(false, turn.is_possible(11, 1, 1)); 
    }


    #[test]
    fn test_min_required() {
        let turn: Turn = " 10 red, 2 green".try_into().unwrap();
        assert_eq!((10, 2, 0), turn.needed_for_turn()); 
        let turn: Turn = " 10 red, 2 green, 19 blue".try_into().unwrap();
        assert_eq!((10, 2, 19), turn.needed_for_turn()); 
    }
    
    #[test]
    fn test_try_from_game() {
        let games : Result<Vec<Game>, String>= TEST.lines().map(|l| l.try_into()).collect();

        assert_eq!(true, games.is_ok());
        assert_eq!(5, games.as_ref().unwrap().len());
        games.as_ref().unwrap().iter().enumerate().for_each(|(pos, game)| {
            assert_eq!((pos + 1) as i64, game.id);
        });
        assert_eq!(Colour::Blue(3), games.as_ref().unwrap()[0].turns[0].colours[0]);

    }

    #[test]
    fn test_game_possible() {
        let games : Vec<Game> = TEST.lines().map(|l| l.try_into().unwrap()).collect();
    
        assert_eq!(true, games[0].is_possible(12, 13, 14));
        assert_eq!(true, games[1].is_possible(12, 13, 14));
        assert_eq!(true, games[4].is_possible(12, 13, 14));
        assert_eq!(false, games[2].is_possible(12, 13, 14));
        assert_eq!(false, games[3].is_possible(12, 13, 14));
    }

    #[test]
    fn test_required() {
        let games : Vec<Game> = TEST.lines().map(|l| l.try_into().unwrap()).collect();


        assert_eq!((4, 2, 6), games[0].required());
        assert_eq!((1, 3, 4), games[1].required());
        assert_eq!((20, 13, 6), games[2].required());
        assert_eq!((14, 3, 15), games[3].required());
        assert_eq!((6, 3, 2), games[4].required());
    }
}
