// i32 is a type of parameter, integer of 32 bits

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn div(a: i32, b: i32) -> f32 {
    // type conversion
    a as f32 / b as f32
}

fn mult(a: f32, b: f32) -> f32 {
    a * b
}

fn main() {
    println!("Hello, I'am a function!");

    let result = sum(4, 4);

    println!("Result of sum: {:?}", result);

    let result = sub(5, 2);

    println!("Result of sub: {:?}", result);

    let result = div(5, 2);

    println!("Result of div: {:?}", result);

    let result = mult(5.5, 2.5);

    println!("Result of div: {:?}", result);
}
