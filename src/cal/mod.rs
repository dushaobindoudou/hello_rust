
pub fn t_t_s(num: &i32, toBase:i32) -> String {
  let mut base = *num;
  if base < 0 {
    base = -num;
  }
  let mut res_str = String::new();
  while base > 0 {
    let val = (base % toBase).to_string();
    let chars:Vec<char> = val.chars().collect();
    for iv in chars {
      res_str.push(iv);
    }
    base = base / toBase;
  }
  println!("规则转换：{}", num);
  res_str.chars().rev().collect::<String>()
}


