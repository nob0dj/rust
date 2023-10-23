
/// from : 다른 타입으로 부터 하나의 타입 생성하기 trait
/// into : from구현시 자동으로 구현. 동일하 결과지만, 다른 타입으로부터 호출하는 방식.
/// trait implementation 규격을 제공하고, 이를 구현하는 방식. 자바 인터페이스와 비슷한듯.
fn main() {
  let my_name = String::from("David");
  println!("{}", my_name); // David
  let my_city: String = "Seoul".into(); // -> String, &str?
  println!("{}", my_city); // Seoul 

  // Number struct 예제
  let n = Number::from(30);
  println!("{} {}", n.num, n.has_point); // 30 false
  let f = Number::from(5.5);
  println!("{} {}", f.num, f.has_point); // 5.5 true

  let n2: Number = 30.into();
  println!("{} {}", n.num, n.has_point); // 30 false
  let f2: Number = 5.5.into();
  println!("{} {}", f2.num, f2.has_point); // 5.5 true
}

struct Number {
  num: String,
  has_point: bool
}

impl From<i32> for Number {
  fn from(item: i32) -> Number {
    Number {
      num: item.to_string(),
      has_point: false
    }
  }
}
impl From<f32> for Number {
  fn from(item: f32) -> Number {
    Number {
      num: item.to_string(),
      has_point: true
    }
  }
}

