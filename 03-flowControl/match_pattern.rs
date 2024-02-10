pub fn match_values() {
    let age = 18;
    println!("My age is: {0}", age);

    match age {
        //match singlr value
        1 => println!("One year old!"),
        //Match sevaral values
        12 | 8 | 15 => println!("Your age is 12 or 8 or 15 now!"),
        //To match the range values
        15..=20 => println!("You are tenager now!"),
        //Handle the rest of cases
        _ => println!("Are you bove 20 ?"),
    }
}