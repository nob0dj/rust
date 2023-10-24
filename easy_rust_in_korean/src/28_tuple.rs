fn main() {
  let my_tuple = (8, "David", vec![1, 2, 3]); // ({integer}, &str, Vec<{integer}>
  println!("{:?}", my_tuple); // (8, "David", [1, 2, 3])
  
  let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: First item: {}
Second item: {}
Third item: {:?}
Fourth item: {}
Fifth item: {:?}
Sixth item: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    );

  // Vec<(String, i32)>
  let my_vec = vec![("Hello", 12), ("Bye", 33)];
  println!("{:?}", my_vec); // [("Hello", 12), ("Bye", 33)]

  // destructuring
  let str_tuple = ("one", "two", "three");
  let (a, b, c) = str_tuple;
  println!("{}, {}, {}", a, b, c); // one, two, three
  let (x, _, _) = str_tuple; // 사용하지 않는 경우, 요소의 개수를 맞춰야 함.
  println!("{}", x); // one

  // array destructuring도 가능하다
  let arr = [1, 2, 3];
  let [k, _, l] = arr;
  println!("{}, {}", k, l); // 1, 3
}