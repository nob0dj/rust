fn main() {
    let a : u8 = 127; // 명시하지않으면 i32 기본타입
    let b = 50; // Type inference u8
    let c = a + b; // 다른 타입간의 연산은 불가능하다.
    println!("a = {a}");
    println!("b = {b}");
    println!("c = {c}");

}
