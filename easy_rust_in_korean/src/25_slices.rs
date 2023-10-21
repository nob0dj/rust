

/// slices
/// - dynamically sized type -> 포인터이용해서 접근
fn main() {

  let seasons = ["봄", "여름", "가을", "겨울"];
  println!("{:?}", seasons); // ["봄", "여름", "가을", "겨울"]
  // println!("{:?}", seasons[0..2]); // error[E0277]: the size for values of type `[&str]` cannot be known at compilation time
  println!("{:?}", &seasons[0..2]); // ["봄", "여름"]
  println!("{:?}", &seasons[0..=2]); // ["봄", "여름", "가을"]
  println!("{:?}", &seasons[..=2]); // ["봄", "여름", "가을"]
}