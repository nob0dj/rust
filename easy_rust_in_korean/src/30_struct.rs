/// 새로운 타입 생성
/// 1. struct - 모든 필드
/// 2. enum - 일부 필드
/// 
/// struct의 종류
/// 1. unit struct - byte수가 0인 marker type
/// 2. tuple struct
/// 3. named struct
#[derive(Debug)] // 특정 struct debug attribute 추가하기
struct FileDirectory;
#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Country {
  name : String, 
  population : u32, 
  capital : String,
  leader_name : String
}

fn main() {
  let x = FileDirectory;
  test_unit_struct(x);

  test_tuple_struct();

  test_named_struct();  
  test_named_struct2();  

}

/// 변수를 간단히 속성으로 등록할 수 있다.
fn test_named_struct2() {
  let name = "한국".to_string();
  let population = 51_000_000; 
  let capital = "서울".to_string();
  let leader_name = "윤석열".to_string();

  let korea = Country {
    name, 
    population,
    capital,
    leader_name
  };
  println!("한국 : {:#?}", korea)
}

fn test_named_struct() {
  let korea = Country {
    name : "한국".to_string(), 
    population : 51_000_000,
    capital : "서울".to_string(), 
    leader_name : "윤석열".to_string()
  };
  println!("한국은 현재 인구가 {}명이고, 수도는 {}, 대통령은 {}입니다.", 
          korea.population, 
          korea.capital, 
          korea.leader_name, 
      );
}

fn test_tuple_struct() {
  let my_color = Color(255, 30, 100);
  println!("My color is {:#?}", my_color);
}

/// unit struct
/// struct FileDirectory
fn test_unit_struct(dir: FileDirectory) {
  println!("I get a file directory! - {:?}", dir); // I get a file directory! - FileDirectory
  println!("The size is {}", std::mem::size_of_val(&dir))
}