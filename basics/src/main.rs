// use rand::Rng;

fn main() {
    // macro_println();
    // semicolon();
    // code_block();
    // generate_random_num();
    mutability();
    shadowing();
}

/// 사용했던 변수를 새로 선언하면서, 기존 변수는 더이상 접근이 불가능하게 하는것.
/// 주로 관심사가 마지막에 있을때 사용하면 좋다.
fn shadowing() {
    let x = 10; 
    let x = double(x); // 20
    let x = triple(x); // 60
    println!("{x}");


    let name = "홍길동";
    println!("{name}"); // 홍길동
    {
        let name = "신사임당";
        println!("{name}"); // 신사임당
    }    
    println!("{name}"); // 홍길동
}

fn double(input : i32) -> i32 {
    input * 2
}
fn triple(input : i32) -> i32 {
    input * 3
}

/// rust immutable by default 
fn mutability() {
    let mut n = 10;
    // n = 9;
    println!("{n}")
}

/// 1. Cargo.toml dependency 설정
/// 2. use rand::Rng 선언
/// 3. rand::thread_rng().get_range(start..end)
/// 4. 터미널에서 cargo build, cargo run 수행 (vscode run은 import 오류ㅠ)
// fn generate_random_num() {
//     let secret_number = rand::thread_rng().gen_range(1..101);
//     println!("사용자가 맞혀야 할 숫자: {}", secret_number);
// }

fn code_block() {
    let num = multiply_nums(5,6);
    println!("{}", num); // 30

    let num = calc(3, 5, 7);
    println!("{}", num); // 126 (10 + 3 + 5) * 7
}

fn calc(m : i32, n : i32, l : i32) -> i32 {
    /// code_block 마지막의 return 값이 변수에 대입된다.
    let result = {
        let k = 10;
        k + m + n
    }; // 마지막 세미콜론 필수
    result * l
}

fn multiply_nums(m : i32, n : i32) -> i32 {
    m * n
}

fn semicolon() {
    // println!("{}", empty_tuple());  // help: the trait `std::fmt::Display` is not implemented for `()`
    
    // empty_tuple처럼 Display할수 없는 경우, Debug print {:?}할 수 있다.
    println!("{:?}", empty_tuple()) // ()
}

/// rust 함수의 void 즉, return 값이 없는 경우 empty_tuple을 반환한다.
/// -> ()
fn empty_tuple() -> (){
    // 8; // 리턴구문이라면 ;이 없어야 한다. ;이 있다면 return값이 아닌 statement.
}

/// macro 
/// - super function
/// - function that writes code
fn macro_println(){
    println!("Hello world");

    let first_name = "Gildong";
    let last_name = "Hong";
    println!("My name is {} {}. And I'm {} years old.", first_name, last_name, get_age());
    println!("My name is {0} {1}. And I'm {2} years old. Hi {0}", first_name, last_name, get_age()); // 인덱스로 대입
    println!("My name is {first_name} {last_name}. And I'm {} years old.", get_age()); // {} 연산, 표현식, 함수호출  불가
}

/// return 키워드 생략 가능
fn get_age() -> i32 {
    // return 33;
    33
}


