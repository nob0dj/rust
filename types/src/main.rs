fn main() {
    type_integer();
    type_char();
    
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
