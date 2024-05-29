use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut count = String::new();
    println!("Enter your No of Values: ");
    io::stdin()
        .read_line(&mut count)?;

    let num: i32 = count.trim().parse()?;

    if num > 2 && num < 100 {
        println!("You count : {}", num);
    } else {
        println!("Invalid input. Please enter a number between 3 and 99.");
    }

    let mut arr = Vec::new();
    println!("Enter list Value below:");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)?;

        let value: usize = input.trim().parse()?;

        arr.push(value);
        if arr.len() == num as usize {
            break;
        }
    }

    println!("List values: {:?}", arr);
    let mut  sum: isize = 0;
    for i in 0..(num-1) as usize {
        let sub:isize = arr[i] as isize - arr[i+1] as isize;
        if sub >= 0{
            sum += sub;
        } else {
            sum += sub.abs();
        }
    }

    println!("Distance is: {}", sum);
    Ok(())

}
