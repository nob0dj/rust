
/// Collection types
/// Array
fn main() {
  let arr1 = ["one", "two"]; // [&str; 2]
  let arr2 = ["one", "two", "three"]; // [&str; 3]
  // println!("{}", arr1 == arr2); // error[E0277]: can't compare `[&str; 2]` with `[&str; 3]`

  let buffer = [0; 128];
  println!("{:?}", buffer); // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

  let months = ["1월", "2월"];
  println!("{}, {}", months[0], months[1]);
  // println!("{}", months[2]); // index out of bounds: the length is 2 but the index is 2

  // 안전한 방식 get -> Option<T> 반환, 여기서는 Option<&&str>
  // Some 값이 있는 경우 | None 값이 없는 경우
  println!("{:?}, {:?}, {:?}", months.get(0), months.get(1), months.get(2)); // Some("1월"), Some("2월"), None
}