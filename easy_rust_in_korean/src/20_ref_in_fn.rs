/// move semantics
/// 함수에 String을 전달하면 ownership도 이전되어 다음 호출시 오류가 발생한다.
/// 
/// Solution1. ref를 전달해서 해결할 수 있다.
/// Solution2. string.clone()
/// Solution3. &str
fn main() {
  let country = "대한민국".to_string();
  print_country(country); // ok
  // print_country(country); // error[E0382]: use of moved value: `country`

  // Solution1. &String
  let car = "소나타".to_string();
  print_car(&car);
  print_car(&car);

  // Solution2. clone
  let food = "김치찌게".to_string();
  print_food(food.clone());
  print_food(food.clone());

  // Solution3. &str
  let coffee = "플랫화이트";
  print_coffee(coffee);
  print_coffee(coffee);
}

fn print_country(country: String) {
  println!("Country is {country}");
}

fn print_car(car: &String) {
  println!("Car is {car}");
}

fn print_food(food: String) {
  println!("Food is {food}"); 
}

fn print_coffee(coffee: &str) {
  println!("Coffee is {coffee}"); 
}