fn main() {
  let input = include_str!("input.txt");
  let output = part1(input);
  dbg!(output);
}

fn part1 (input: &str) -> i32 {

  let answer: i32 = input.lines().fold(0, |acc: i32, line: &str| {
    let nums = line.char_indices().fold("".to_string(), |acc, (i, c)| {
      if c.is_digit(10) {
        acc + &c.to_string()
      } else {
        let mut value = "";
        let substring = &line[i..];
        if substring.starts_with("zero") {value = "0"}
        else if substring.starts_with("one")  {value = "1"}
        else if substring.starts_with("two")  {value = "2"}
        else if substring.starts_with("three"){value = "3"}
        else if substring.starts_with("four") {value = "4"}
        else if substring.starts_with("five") {value = "5"}
        else if substring.starts_with("six")  {value = "6"}
        else if substring.starts_with("seven"){value = "7"}
        else if substring.starts_with("eight"){value = "8"}
        else if substring.starts_with("nine") {value = "9"}
        acc+value
      }
    });
    let partial: i32 = (format!("{}{}",nums.chars().next().unwrap(), nums.chars().last().unwrap())).parse::<i32>().unwrap();
    acc + partial
  });
  answer
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let test_input = include_str!("test.txt");
    assert_eq!(part1(test_input), 142);
  }
}
