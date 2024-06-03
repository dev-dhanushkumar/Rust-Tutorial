use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut count = String::new();
    println!("How many values to enter? :");
    io::stdin()
        .read_line(&mut count)?;

    let num: i32 = count
        .trim()
        .parse()?;
    
    if num > 0 && num < 100 {
        println!("Numbert of items in Vector: {}", num);
    } else {
        println!("Please enter number between 1 and 99");
    }

    let mut arr = Vec::new();
    
    println!("Enter vector values Below:");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)?;
        
        let value: usize = input.trim().parse()?;

        arr.push(value);
        let len = vector_length(&arr);
        if len == num as usize {
            break;
        }
    }

    println!("Array values: {:?}", arr);
    odd_or_enen_vec(&mut  arr);
    Ok(())
}

fn vector_length<T>(vec: &[T]) -> usize {
    if vec.is_empty() {
        0
    } else {
        1 + vector_length(&vec[1..])
    }
}

fn odd_or_enen_vec(v: &mut Vec<usize>) {
    
    let mut odd = Vec::new();
    let mut even = Vec::new();

    for num in v.iter() {
        if num % 2 == 0 {
            even.push(*num);
        } else {
            odd.push(*num);
        }
    }
    let odd_len = vector_length(&odd);
    let even_len = vector_length(&even);
    println!("Length of odd vector: {} and values are {:?}", odd_len,selection_sort(&mut odd));
    println!("Length of even vector: {} and values are {:?}", even_len, selection_sort(&mut even));
}

fn selection_sort(arr: &mut Vec<usize>) -> Vec<usize>{
    let len = vector_length(&arr);
    for i in 0..len-1 {
        let mut min_idx = i;
        for j in i+1..len {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }

        let temp = arr[i];
        arr[i] = arr[min_idx];
        arr[min_idx] = temp;
        // println!("Sorting Processing case {}: {:?}",i, arr);
    }
    arr.to_vec()
}