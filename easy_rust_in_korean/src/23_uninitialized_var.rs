fn main() {
  // let num; // error[E0282]: type annotations needed
  let num: i32; // error[E0381]: used binding `num` isn't initialized
  // num = 123; // 값대입후 사용가능!

  {
    num = factorial(5);
  }  
  println!("num is {num}");
}

fn factorial(mut n: i32) -> i32 {
  let mut result = 1;
  loop {
    result *= n;
    n -= 1;
    println!("{n} {result}");
    if n == 1 {
      break;
    }
  }
  result
}