fn main() {
  test_if();
  test_match();
  test_match2();
}

/// tuple match문에 사용하기
fn test_match2() {
  let sky = "cloudy";
  let temperature = 20;
  
  match (sky, temperature) {
    ("cloudy", 0) => println!("It's cloudy and freezy!"),
    ("cloudy", 10) => println!("It's cloudy and cold!"),
    ("cloudy", 20) => println!("It's cloudy and warm!"),
    ("clear", 20) => println!("It's clear and warm!"), 
    _ => println!("I don't know!"),
  }
}


fn test_match() {
  let n = 3;

  match n {
    0 => println!("0"),
    1 => println!("1"),
    2 => println!("2"),
    _ => println!("other number!")
  }

  let m = match n {
    0 => 0,
    1 => 11,
    2 => 22,
    _ => 999,
  }; // 세미콜론 필수
  println!("{}", m); // 999
}

/// 조건식 && || 사용가능
fn test_if() {
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