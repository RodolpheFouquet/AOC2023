
const TEST: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

const TEST_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

fn main() {
  let s : i64 = include_str!("../input.txt").lines().map(|l| get_val(l).unwrap()).sum();
  let s2 : i64 = include_str!("../input2.txt").lines().map(|l| get_val_with_strings(l.as_bytes()).unwrap()).sum();

  println!("{}", s);
  println!("{}", s2);
}


fn get_val(line: &str) -> Result<i64, std::num::ParseIntError> {
    let numbers = line.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<char>>();
    vec![numbers[0], numbers[numbers.len() -1]].iter().collect::<String>().parse::<i64>()
}


fn get_val_with_strings(line: &[u8]) -> Result<i64, std::num::ParseIntError> { 
    let mut digits: Vec<char> = Vec::new(); 
    for i in 0..line.len() {
        let c = &line[i];
        if (*c as char).is_ascii_digit() {
            digits.push(*c as char);
        }

        let substr = std::str::from_utf8(&line[i..line.len()]).unwrap();

        if substr.starts_with("one") {
            digits.push('1');
        } else if substr.starts_with("two") {
            digits.push('2');
        } else if substr.starts_with("three") {
            digits.push('3');
        } else if substr.starts_with("four") {
            digits.push('4');
        } else if substr.starts_with("five") {
            digits.push('5');
        } else if substr.starts_with("six") {
            digits.push('6');
        } else if substr.starts_with("seven") {
            digits.push('7');
        } else if substr.starts_with("eight") {
            digits.push('8');
        } else if substr.starts_with("nine") {
            digits.push('9');
        }
        
    }
    vec![digits[0], digits[digits.len() -1]].iter().collect::<String>().parse::<i64>()
}


#[cfg(test)]
mod tests { 

    use super::*;

    #[test]
    fn test_get_value() {
        let test_values: Vec<i64> = TEST.lines().map(|l| get_val(l).unwrap()).collect();
        assert_eq!(test_values[0], 12);
        assert_eq!(test_values[1], 38);
        assert_eq!(test_values[2], 15);
        assert_eq!(test_values[3], 77);
    }

    #[test]
    fn test_replace_digit() {
        let test_values: Vec<i64> = TEST_2.lines().map(|l| get_val_with_strings(l.as_bytes()).unwrap()).collect();

        assert_eq!(test_values[0], 29);
        assert_eq!(test_values[1], 83);
        assert_eq!(test_values[2], 13);
        assert_eq!(test_values[3], 24);
        assert_eq!(test_values[4], 42);
        assert_eq!(test_values[5], 14);
        assert_eq!(test_values[6], 76);
    }
}
