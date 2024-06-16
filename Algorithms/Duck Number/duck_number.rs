use std::io;
use std::error:: Error;


fn user_input() -> Result<i32, Box<dyn Error>> {
    let mut count = String::new();
    println!("Enter the No of Values:");
    io::stdin()
        .read_line(&mut count)?;
    let num : i32 = count.trim().parse()?;
    Ok(num)
}

fn is_duck_number(mut num: i32) -> bool {
    let mut rem;
    loop {
        rem = num % 10;
        if rem == 0 {
            return true;
        }
        num /= 10;
        if num < 0 {
            break;
        }
    }
    return false;
}

fn main() -> Result<(), Box<dyn Error>> {
    let  num: i32 = user_input()?;
    // println!("You enter Number is: {}", num);
    if is_duck_number(num) {
        println!("{} is Duck Number", num);
    } else {
        println!("{} is not Duck Number", num);
    }
    Ok(())
}

