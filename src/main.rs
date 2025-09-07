mod ownership;

fn main() {
    println!("Hello, world!");
    variable_declaration();
    control_statement();
    let sum = add(5, 10);
    println!("Sum: {}", sum);
    ownership::ownership_example();
}

fn variable_declaration() {
    let a: i32 = 42; // 불변 변수
    let b: f64 = 3.14;
    let c: &str = "Hello, Rust!";
    let mut d = 10; // 가변 변수
    d += 5;
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
}

fn control_statement() {
    let number = 10;

    if number < 5 {
        println!("Number is less than 5");
    } else if number == 5 {
        println!("Number is equal to 5");
    } else {
        println!("Number is greater than 5");
    }

    for i in 0..5 { // 0부터 4까지 반복 (inclusive start, exclusive end)
        println!("i: {}", i);
    }

    let mut count = 0;
    while count < 5 {
        println!("count: {}", count);
        count += 1;
    }
}

fn add(a: i32, b: i32) -> i32 {
    println!("a: {}, b: {}", a, b);
    a + b               // 암시적 반환
    // return a + b;    // 명시적 반환
}
