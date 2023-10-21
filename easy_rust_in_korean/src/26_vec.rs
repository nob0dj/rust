/// Vec<T>
/// 타입지정 필수, 자료형을 명시하던지 값 하나를 대입후 사용 
fn main() {
  // let my_vec = Vec::new();
  // println!("{:?}", my_vec); // error[E0282]: type annotations needed for `Vec<T>`
  
  let my_vec: Vec<String> = Vec::new();
  println!("{:?}", my_vec); // []

  let name1 = String::from("Windy");
  let name2 = String::from("Gomesy");

  // mut지정을 통해 값추가 가능
  // capacity : vector가 reallocating없이 가질수 있는 요소수 0, 4, 8, 16...
  let mut my_vec = Vec::new(); // Vec<_>
  println!("capacity : {}, len : {}", my_vec.capacity(), my_vec.len()); // 0, 0
  my_vec.push(name1); // Vec<String>
  println!("capacity : {}, len : {}", my_vec.capacity(), my_vec.len()); // 4, 1
  my_vec.push(name2);
  println!("capacity : {}, len : {}", my_vec.capacity(), my_vec.len()); // 4, 2

  println!("My cats are {:?}", my_vec); // My cats are ["Windy", "Gomesy"]


  println!("{}", my_vec[0]); // Windy
  println!("{}", my_vec[1]); // Gomesy
  println!("{:?}", my_vec.get(0)); // Some("Windy")
  println!("{:?}", my_vec.get(1)); // Some("Gomesy")


  // 매크로로 Vec 생성하기
  let vecs = vec![10, 20, 30]; // Vec<{integer}>
  println!("{:?}", vecs); // [10, 20, 30]
  println!("{}", vecs.len()); // 3
  
}