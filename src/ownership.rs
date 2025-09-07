pub fn ownership_example() {
    let s1 = String::from("hello");
    let s2 = s1; // 소유권이 s2로 이동(move)
    // println!("{}", s1); // 오류! s1은 더 이상 사용할 수 없음
    println!("{}", s2); // OK

    let x = 5;
    let y = x; // i32는 Copy 트레잇이 있어서 복사됨
    println!("x: {}, y: {}", x, y); // OK
}

pub fn borrowing_example() {
    let s = String::from("hello");
    let r1 = &s; // 불변 참조
    let r2 = &s; // 또 다른 불변 참조
    // let r3 = &mut s; // 에러! 불변 참조가 있을 때 가변 참조 불가
    println!("r1: {}, r2: {}", r1, r2);
}

pub fn mutable_borrowing_example() {
    let mut s = String::from("hello");
    let r1 = &mut s; // 가변 참조
    r1.push_str(", world");
    println!("r1: {}", r1);
    // let r2 = &s; // 에러! 가변 참조가 있을 때 불변 참조 불가
}

pub fn slice_example() {
    let s = String::from("hello");
    let slice = &s[0..2]; // "he"
    println!("slice: {}", slice);
}