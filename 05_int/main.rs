/*
整数可以分为 有符号整型 和 无符号整型

有符号整型，英文 signed，既可以存储正数，也可以存储负数。
无符号整型，因为 unsigned，只能存储正数。

*/
fn main(){
  let price = 100.0011212;
  let price2:u32 = 200;
  let price3:i32 = -300;
  let price4:isize = 400;
  let price5:usize = 500;
  println!("price is {}", price);
  println!("price2 is {} and price3 is {}", price2, price3);
  println!("price4 is {} and price5 is {}", price4, price5);

  // println!("std::u128:MAX is {}", std::u128:MAX);
}
