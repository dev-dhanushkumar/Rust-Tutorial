use std::io;
use std::error::Error;

fn odd_or_even_game(v: &mut Vec<usize>) {
    println!("Function Vector: {:?}", v);

    let mut odd = Vec::new();
    let mut even = Vec::new();

    for num in  v.iter() {
        if num % 2 == 0 {
            even.push(*num);
        } else {
            odd.push(*num);
        }
    }
    v.clear();
    v.append(&mut even);
    v.append(&mut odd);
}

fn main() -> Result<(), Box<dyn Error>>{
    let mut count = String::new();
    println!("How many values to enter? :");
    io::stdin()
        .read_line(&mut count)?;

    let num : i32 = count
        .trim()
        .parse()?;

    if num > 0 && num < 100 {
        println!("Numbert of items in Vector: {}", num);
    } else {
        println!("Please enter number between 1 and 99");
    }

    let mut arr = Vec::new();
    println!("Enter Vector Values Below:");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)?;
        
        let value:usize = input.trim().parse()?;

        arr.push(value);
        if arr.len() == num as usize {
            break;
        }
    }

    println!("Vector values: {:?}", arr);
    odd_or_even_game(&mut arr);
    println!("{:?}", arr);
    Ok(())
}