/// api doc comments

fn main() {
    println!("Hello, world!");
    // 한줄 주석
    /* 
        여러줄 주석
    */
    let _x/* : i64 */= 10; 
    // 선언한 변수를 사용하지 않으면 warning: unused variable: `x`
    // 변수 접두사 _를 사용하면 무시하라는 뜻
    // println!("{}", x)
}
