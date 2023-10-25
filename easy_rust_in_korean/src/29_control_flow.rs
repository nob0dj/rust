/// 조건식 && || 사용가능
fn main() {
  let num = 31;
  // 조건식괄호는 필수가 아니다.
  if num == 0 {
    println!("짝수");
  }
  else if num % 2 == 0 {
    println!("짝수");
  }
  else {
    println!("홀수");
  }
}