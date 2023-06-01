fn main(){

    let decision : bool = true;

    if decision {
        println!("Oh Yeah, I'm here!");
    } else {
        println!("Oh, I fell into the else");
    }

    let decision_two = 3;

    if decision_two == 1{
        println!("I'm number one!");
    } else if decision_two == 2{
        println!("I'm number two!");
    } else {
        println!("I'm other number!");
    }

    let variable_for_match = "any";

    match variable_for_match {
        "do_match" => println!("Do not match!"),
        "match" => println!("MATCH!!!!"),
        _ => println!("Any"),
    }

    
}