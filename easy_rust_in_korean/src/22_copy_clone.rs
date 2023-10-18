/// copy types : call by value처럼 작동 i32 
///   - It's trivial to copy bytes!
/// non-copy type
///   - copy type처럼 사용하기 위해 clone을 사용할 수 있다.
fn main() {
  // copy type
  let num = 33;
  print_num(num);
  print_num(num);

  // none-conpy type
  // ownership이전 관련 issue를 처리하기 위해 clone 호출
  let name = "홍길동".to_string();
  print_name(name.clone());
  print_name(name.clone());
}

fn print_num(num: i32) {
  println!("num is {num}")
}

fn print_name(name: String) {
  println!("name is {name}");
}