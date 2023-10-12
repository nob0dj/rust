fn main() {
    macro_println();
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

