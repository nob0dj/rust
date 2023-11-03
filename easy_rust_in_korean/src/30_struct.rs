/// 새로운 타입 생성
/// 1. struct - 모든 필드
/// 2. enum - 일부 필드
/// 
/// struct의 종류
/// 1. unit struct - byte수가 0인 marker type
/// 
struct FileDirectory;


fn main() {
  let x = FileDirectory;
  test_unit_struct(x);
}

/// unit struct
/// struct FileDirectory
fn test_unit_struct(dir: FileDirectory) {
  println!("I get a file directory!"); // dir 출력불가
  println!("The size is {}", std::mem::size_of_val(&dir))
}