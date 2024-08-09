fn poppin_time(stack: &mut Vec<String>, repeat_count: i32, c: Option<&String>) -> i32 {
  let mut done = false;
  let mut new_repeat_count = repeat_count;
  while !done {
    done = true;
    match stack.last() {
      None => if let Some(c) = c { stack.push(c.clone()) },
      Some(top_of_stack) => {
        if Some(top_of_stack) == c { new_repeat_count += 1; }
        else {
          match (new_repeat_count, c) {
            (0, Some(c)) => stack.push(c.clone()),
            (0, None) => {},
            _ => { stack.pop(); done = false; },
          }
          new_repeat_count = 0;
        }
      },
    }
  }

  new_repeat_count
}

pub fn pop_blocks(lst: &[String]) -> Vec<String> {
  println!("Input: {lst:?}");
  let mut stack: Vec<String> = Vec::new();
  let mut repeat_count = 0;
  for c in lst {
    repeat_count = poppin_time(&mut stack, repeat_count, Some(c));
  }

  poppin_time(&mut stack, repeat_count, None);

  stack
}





#[cfg(test)]
mod tests {
  use super::pop_blocks;
      
  fn dotest(lst: &[String], expected: &[String]) {
    let actual = pop_blocks(lst);
    assert!(actual == expected, 
      "With lst = {lst:?}\nExpected {expected:?} but got {actual:?}")
  }

  #[test]
  fn basic_test_cases() {
    dotest(&[], &[]);
    dotest(&["B".to_string(), "B".to_string(), "A".to_string(), "C".to_string(), "A".to_string(), "A".to_string(), "C".to_string()], &["A".to_string()]);
    dotest(&["B".to_string(), "B".to_string(), "C".to_string(), "C".to_string(), "A".to_string(), "A".to_string(), "A".to_string()], &[]);
    dotest(&["C".to_string(), "A".to_string(), "C".to_string()], &["C".to_string(), "A".to_string(), "C".to_string()]);
    dotest(&["C".to_string(), "A".to_string(), "A".to_string(), "C".to_string(), "B".to_string()], &["B".to_string()]);
    dotest(&["ab".to_string(), "ab".to_string(), "cd".to_string(), "cx".to_string(), "B".to_string()], &["cd".to_string(), "cx".to_string(), "B".to_string()]);
    dotest(&["C".to_string(), "C".to_string()], &[]);
    dotest(&["A".to_string(), "B".to_string(), "C".to_string(), "C".to_string(), "B".to_string(), "D".to_string(), "A".to_string()], &["A".to_string(), "D".to_string(), "A".to_string()]);
    dotest(&["A".to_string(), "B".to_string(), "A".to_string(), "A".to_string(), "A".to_string(), "B".to_string(), "B".to_string()], &["A".to_string()]);
    dotest(&["D".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "D".to_string(), "D".to_string(), "C".to_string(), "B".to_string(), "C".to_string(), "D".to_string()], &["D".to_string(), "C".to_string(), "D".to_string()]);
  }
  
  #[test]
  fn edge_cases() {
    dotest(&["A".to_string(), "A".to_string(), "AB".to_string()], &["AB".to_string()]);
    dotest(&["A".to_string(), "B".to_string(), "B".to_string(), "A".to_string(), "AB".to_string()], &["AB".to_string()]);
    dotest(&["A".to_string(), "AB".to_string(), "AB".to_string(), "A".to_string()], &[]);
  }
}
