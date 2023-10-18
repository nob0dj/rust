fn main() {
  let mut country = "한국".to_string();
  concat_str(&mut country); // 한국ㅋ
  concat_str(&mut country); // 한국ㅋㅋ


  // 매개인자 String -> 매개변수 mut (ownership 이전 없음)
  let name = "홍길동".to_string();
  update_name(name); // 매개인자 String
}

fn concat_str(str: &mut String) {
  str.push_str("ㅋ");
  println!("Now str is {str}")
}

/// 매개인자 String을 받아서 mut 변수로 새로 선언해서 사용
fn update_name(mut name: String) {
  name.push_str("ㅎㅎㅎ");
  println!("Now name is {name}");
}