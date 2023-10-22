
/// from : 다른 타입으로 부터 하나의 타입 생성하기 trait
/// into : from구현시 자동으로 구현. 동일하 결과지만, 다른 타입으로부터 호출하는 방식.
/// trait implementation 규격을 제공하고, 이를 구현하는 방식. 자바 인터페이스와 비슷한듯.
fn main() {
  let my_name = String::from("David");
  println!("{}", my_name); // David
  let my_city: String = "Seoul".into(); // -> String, &str?
  println!("{}", my_city); // Seoul 
}

