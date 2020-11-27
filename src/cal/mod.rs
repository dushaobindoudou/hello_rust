
pub fn t_t_s(num: i32, toBase:i32) -> String {
  let mut base = 0;
  if base < 0 {
    base = -num;
  } else {
    base = num;
  }

  let mut res_str = String::new();
  while base > 0 {
    let val = (base % toBase).to_string();
    let chars:Vec<char> = val.chars().collect();
    for iv in chars {
      res_str.push(iv);
    }
    // res_str.push(val);
    base = base / toBase;
  }
  println!("规则转换：{}, {:?}", num, res_str);
  res_str.chars().collect::<String>()
}


