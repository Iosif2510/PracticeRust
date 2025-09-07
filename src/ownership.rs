pub fn ownership_example() {
    let s1 = String::from("hello");
    let s2 = s1; // 소유권이 s2로 이동(move)
    // println!("{}", s1); // 오류! s1은 더 이상 사용할 수 없음
    println!("{}", s2); // OK

    let x = 5;
    let y = x; // i32는 Copy 트레잇이 있어서 복사됨
    println!("x: {}, y: {}", x, y); // OK
}