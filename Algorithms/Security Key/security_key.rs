#![allow(unused)]

use std::collections::HashMap;

fn find_security_key(data: i64) -> i32 {
    //convert the integer data to string format
    let data_str = data.to_string();
    println!("{:?}", data_str);

    let mut digit_count = HashMap::new();

    //count the each digit
    for digit in data_str.chars() {
        let counter = digit_count.entry(digit).or_insert(0);
        *counter += 1;
    }

    println!("{:?}", &digit_count);

    //Vector to store the repeated value
    let mut repeated_value = Vec::new();

    // Identify the repeated Digit 
    for (&digit , &count) in digit_count.iter() {
        if count  > 1{
            repeated_value.push(digit.to_digit(10).unwrap());
        }
    }

    println!("{:?}", repeated_value);
    if repeated_value.len() != 0 {
        repeated_value.len() as i32
    }else {
        -1
    }

    // if let Some(&min_digit) = repeated_value.iter().min() {
    //     min_digit as i32 
    // }else {
    //     -1
    // }
}

fn main() {
    let data = 578378923;
    let output  = find_security_key(data);
    println!("Security Key: {}", output);
}