fn main() {
  let input = include_str!("input.txt");
  let output = part1(input);
  dbg!(output);
}

fn part1 (input: &str) -> i32 {
  let answer: i32 = input.lines().fold(0, |acc: i32, line: &str| {
    let mut index = 0;
    let mut mul:i32 = 10;
    let nums = line.char_indices().fold([872377 as i32;2], |mut acc, (i, c)| {
      if acc[0] != 872377 {
        index = 1;
        mul = 1;
      }
      if c.is_digit(10) {
          acc[index] = i as i32*mul;
          acc
      }
      else {
        let substring = &line[i..];
        if substring.starts_with("zero")      {acc[index] = 0 as i32*mul;}
        else if substring.starts_with("one")  {acc[index] = 1 as i32*mul;}
        else if substring.starts_with("two")  {acc[index] = 2 as i32*mul;}
        else if substring.starts_with("three"){acc[index] = 3 as i32*mul;}
        else if substring.starts_with("four") {acc[index] = 4 as i32*mul;}
        else if substring.starts_with("five") {acc[index] = 5 as i32*mul;}
        else if substring.starts_with("six")  {acc[index] = 6 as i32*mul;}
        else if substring.starts_with("seven"){acc[index] = 7 as i32*mul;}
        else if substring.starts_with("eight"){acc[index] = 8 as i32*mul;}
        else if substring.starts_with("nine") {acc[index] = 9 as i32*mul;}
        acc
      }
    });
    print!("{:?} ", nums);
    let partial: i32 = nums.iter().sum::<i32>();
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
