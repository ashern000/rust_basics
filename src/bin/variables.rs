fn main() {
    let normal_variable = 4;

    println!("This is a normal variable! {:?}", normal_variable);

    let mut variable_mutable = 3;

    println!("This is a mutable variable! {:?}", variable_mutable);

    // This is a type of repeating structure and decision, have a more exemples in repeating.rs decision_operators.rs

    loop {
        println!("Yes, I'am mutable {:?}", variable_mutable);

        if variable_mutable > 5 {
            break;
        }

        variable_mutable += 1;
    }


    let number_integer : i32 = 4;

    println!("This is a integer number {:?}", number_integer);

    let number_float : f64 = 3.1415926;

    println!("This is a float number {:?}. 'Oh, I'm number PI'", number_float);


    // In rust, there are only types f32 and f64, for int it has types i8, i16, i32, i64, i128

    let boolean : bool = true;

    println!("I'm a Bool, {:?}?", boolean);

    let boolean : bool = false;

    println!("No, you are not true, you are {:?}", boolean);

    // ATTENTION, char in rust is declared using ''

    let char_variable : char = 'A';

    println!("I'm a char variable, {:?}", char_variable);

    let array_type = [1,2,3,4,5];

    println!("Array: {:?}", array_type);

    // A array the lenght four and integer type

    let array_type : [i32; 4] = [1,2,3,4];

    println!("Array: {:?}", array_type);

    // Array of one type element and lenght four

    let array_type = [3; 5];

    println!("Array of three's: {:?}", array_type);
}
