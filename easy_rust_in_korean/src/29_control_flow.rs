fn main() {
  // test_if();
  // test_match();
  // test_match2();
  // test_match3();
  test_match4();
}

fn test_match4() {
  match_color((10, 100, 100));
  match_color((100, 10, 100));
  match_color((100, 100, 10));
  match_color((100, 100, 100));

}

fn match_color(rgb: (u32, u32, u32)) {
  match rgb {
    (r, _, _) if r < 50 => println!("Redish~"),
    (_, g, _) if g < 50 => println!("Greenish~"),
    (_, _, b) if b < 50 => println!("Bluish~"),
    _ => println!("Some other color!")
  }
}

/// match문에 if사용하기
fn test_match3() {
  let children = 5;
  let married = true;

  match(children, married) {
    (c, m) if !married => println!("Not married!"),
    (c, m) if married && c == 0 => println!("No children!"),
    _ => println!("Has chilren!")
  }
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

  // if문 적용시
  match (sky, temperature) {
    ("cloudy", t) if t < 10 => println!("It's cloudy and freezy!"),
    ("cloudy", t) if t >= 10 && t < 20 => println!("It's cloudy and cold!"),
    ("cloudy", t) if t >= 20 => println!("It's cloudy and warm!"),
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