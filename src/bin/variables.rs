fn main() {
    let normal_variable = 4;

    println!("This is a normal variable! {:?}", normal_variable);

    let mut variable_mutable = 3;

    println!("This is a mutable variable! {:?}", variable_mutable);

    // This is a type of repeating structure, have a more exemples in repeating.rs

    loop {
        println!("Yes, I'am mutable {:?}", variable_mutable);

        if variable_mutable > 5 {
            break;
        }

        variable_mutable += 1;
    }


    let number_integer : i32 = 4;

    println!("This is a integer number {:?}", number_integer);

    let number_float : f32 = 3.1415926;

    println!("This is a float number {:?}. 'Oh, I'm number PI'", number_float);
}
