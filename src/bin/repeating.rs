fn main() {
    // Repetition Structures

    let mut n: i32 = 0;

    while n <= 10 {
        println!("while Loop, number of interation: {:?}", n);
        n += 1;
    }

    let mut l: i32 = 0;

    loop {
        println!("Loop, loooooping!");

        if l == 5 {
            break;
        }
        l += 1;
    }

    for n in 1..=10 {
        println!("Number of interation: {:?}", n);
    }
}
