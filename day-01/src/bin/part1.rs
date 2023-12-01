fn main() {
  let input = include_str!("input.txt");
  let output = part1(input);
  dbg!(output);
}

fn part1 (input: &str) -> i32 {
  let answer = input.lines().fold(0, |acc, line| {
    let nums = line.chars().filter(|c| c.is_digit(10)).collect::<String>();
    let partial = (format!("{}{}",nums.chars().next().unwrap(), nums.chars().last().unwrap())).parse::<i32>().unwrap();
    acc + partial
  });
  answer
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    let test_input = include_str!("test.txt");
    assert_eq!(part1(test_input), 142);
  }
}
