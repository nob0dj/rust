fn main() {
    // macro_println();
    semicolon();
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

