fn main() {
    // type_integer();
    // type_char();
    casting();
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
