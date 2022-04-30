fn main() {
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("spaces length: {}", spaces);
    // [error] 타입 오류 spaces 는 문자열이여야 함

    let spaces = "   ";
    let spaces = spaces.len();
    println!("space length: {}", spaces);

    // let guess = "42".parse().expect("Not a number!");
    // [error] 무슨 타입으려 파싱을 할지 명시되어 있지 않음

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess number: {}", guess);

    // Scala type variable
    // Number: i[bit], u[bit]
    // i = signed, u = unsigned
    // isize, usize = os type에 따라 64bit, 32bit 할당

    // f[32,64]
    // f32 = float32, f64 = float64
    // 등등 기본적인 자료형 구조가 있음

    // 복합 타입 (Tuple / 튜플)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    // tup[1] 같은 배열 값 조회는 오류
    // tuple elements는 tup.0과 같은 형태로 조회 필요
    println!("The value of x, y, z is: {}, {}, {}", x, y, z);
    println!("variable access from tup: {}, {}, {}", tup.0, tup.1, tup.2);

    // 배열
    let arr_1 = [1, 2, 3, 4, 5];
    println!("array print: {}", arr_1[0]);
}
