pub fn solution(s: &str) -> Vec<String> {
  let mut v: Vec<String> = Vec::new();

  let mut count = 0;
  let mut acc: String = String::new();
  for c in s.chars() {
    acc.push(c);
    count += 1;
    if count == 2 {
      v.push(acc.clone());
      acc.clear();
      count = 0;
    }
  }

  if count == 1 {
    acc.push('_');
    v.push(acc.clone());
  }

  v
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
    assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
    assert_eq!(solution(""), [] as [&str; 0]);
  }
}


