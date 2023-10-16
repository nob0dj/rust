#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#[allow(non_upper_case_globals)]

use std::mem::size_of;


fn main() {
    // type_integer();
    // type_char();
    // casting();
    // type_string();
    // type_inference();
    // type_float();

    // type_string2();
    // const_static();
    ownership();
}

/// & immutable reference(shared reference)
/// &mut mutalbe reference / unique reference
fn ownership() {
    let country = String::from("대한민국");
    // 읽기 전용 참조는 개수 제한 없이 생성가능
    let ref_one = &country;
    let ref_two = &country;
    println!("{}, {}", ref_one, ref_two);

    // 함수하위 빌린 참조를 반환할 수 없다.
    // let country = get_country();
    // println!("{}", country);

    // 쓰기전용 참조는 &mut 변수명으로 작성! - &mut와 이후 *값참조의 개수는 동일해야 한다.
    // &mut &mut 원조값 - **포인터
    // 단하나의 쓰기전용 참조만 가질 수 있다.
    let mut num = 9;
    let num_ref = &mut num;
    *num_ref *= 2;
    println!("{num}") // 18
}

/// 지역변수에 대한 포인터를 리턴할 수 없다. 함수 생명주기가 끝날때 해당데이터는 더이상 접근불가!
// fn get_country() -> &String {
//     let country = String::from("캐나다");
//     &country
// }

// 전역변수는 let binding이 아닌 const | static 사용
const HIGH_SCORE: i32 = 20; // global scope 대문자로 작성, 타입명시
static mut LOW_SCORE: i32 = 0; // static은 mutable 가능, unsafe(비추), 프로그램시작~끝 생명주기


fn const_static() {
    println!("{HIGH_SCORE}"); // 20
    // unsafe는 데이터 보장이 안되므로 사용금지
    unsafe {
        println!("{LOW_SCORE}"); // 0
    }
}

/// String owned string
/// &str
/// string
/// slice
fn type_string2() {
    // &str
    let my_name = "David"; // &str
    println!("{my_name}");

    // String
    let my_name = my_name.to_string();
    let my_name = String::from("Bell");
    println!("{my_name}");
    
    // growable/shrinkable string
    let mut my_name = "홍길동".to_string();
    my_name.push('홍'); // 한글자씩 맨뒤에 추가
    my_name.push_str("길동길동");
    println!("{my_name}"); // 홍길동요
    
    // length | capacity 
    // 길이가 늘어날때 마다 reallocation된다. x2
    let mut text = "".to_string();
    println!("length is {}, capacity is {}", text.len(), text.capacity()); // 0 0
    text.push_str("David");
    println!("length is {}, capacity is {}", text.len(), text.capacity()); // 5 8
    text.push_str(" Hi!");
    println!("length is {}, capacity is {}", text.len(), text.capacity()); // 9 16
    text.push_str(" He loves Seoul~");
    println!("length is {}, capacity is {}", text.len(), text.capacity()); // 25 32

    // with_capacity
    // reallocation 없이 한번에 할당하기. 크기가 커지면 reallocation된다.
    let mut text = String::with_capacity(32);
    println!("length is {}, capacity is {}", text.len(), text.capacity()); // 0 32
    text.push_str("David");
    println!("length is {}, capacity is {}", text.len(), text.capacity()); // 5 32
    text.push_str(" Hi!");
    println!("length is {}, capacity is {}", text.len(), text.capacity()); // 9 32
    text.push_str(" He loves Seoul~");
    println!("length is {}, capacity is {}", text.len(), text.capacity()); // 25 32

}

/// 실수 타입 
/// - f32 
/// - f64(기본값)
fn type_float() {
    let f1 = 0.1234567890_f32;
    let f2 = 0.12345678901234567890;
    println!("{}", f1); // 0.12345679 유효자리수 7자리
    println!("{}", f2); // 0.12345678901234568 유효자리수 16자리


    let my_float = 5.; // 5.0
    let my_float = 0.5; // .5  작성불가
    println!("{}", my_float); 

    let other_number = 5;

    println!("{}", my_float as i32 + other_number); // 5
    println!("{}", my_float + other_number as f64); // 5.5

}

fn type_inference() {
    let small_number: u8 = 10;
    let small_number = 10u8;
    let small_number = 10_u8;
    let small_number = 10_______________________u8; // compiler는 _무시
    println!("{}", small_number); // 10

    let big_number = 100_000_000_i32; // 3자리 그룹핑 가능
    println!("{}", big_number); // 100000000
}

/// string
/// string.len() byte수
/// string.chars().count() 글자수
fn type_string() {
    // use 구문 사용하기
    println!("Size of a char: {}", size_of::<char>()); // 4 bytes
    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len()); 
    println!("Size of string containing '𓅱': {}", "𓅱".len());

    let slice = "Hello!"; // 6bytes, 6글자
    println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
    let slice2 = "안녕!"; // 7bytes(한글 각 3bytes), 3글자
    println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
}

/// simple type changing using 'as'
fn casting() {
    let a : u8 = 10;
    let b : u16 = 20;
    let c = a + b as u8; // u8 + u16 불가능하지만, 두번째 인수를 u8으로 type casting한 후에는 가능하다.
    println!("c = {c}");

    let ch : u8 = 97; // u8타입만 char로 casting 가능하다.
    println!("ch = {}", ch as char);

    let n = 'a' as u8; // 97
    let n = '한' as u32; // 54620
    println!("n = {}", n);
}

fn type_char() {
    // "string"
    // 'A' char 4bytes
    let first_letter = 'A';
    let space = ' '; // A space inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face = '😺'; // Emojis are chars too

    println!("{first_letter}");
    println!("'{space}'");
    println!("{other_language_char}");
    println!("{cat_face}");
}

fn type_integer() {
    let a : u8 = 127; // 명시하지않으면 i32 기본타입
    let b = 50; // Type inference u8
    let c = a + b; // 다른 타입간의 연산은 불가능하다.
    println!("a = {a}");
    println!("b = {b}");
    println!("c = {c}");
}
