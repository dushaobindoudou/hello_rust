
pub fn how_much(num:i32) -> i32 {
  let mut count = 0;
  for val in 1..=num {
    let mut i = val;
    while i > 0 {
      if i % 5 == 0 {
        count += 1;
        i = i / 5;
      } else {
        i = 0;
      }
    }
  }
  count
}


