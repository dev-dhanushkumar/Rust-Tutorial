//Using panic! macro to handle this error
/*
fn main(){
    let num = 15;
    if num%2 == 0{
        println!("Even number!");
    }else{
        panic!("Please enter even num!");
    }
}
*/

//HANDLE ERROR USING ENUM
/*
fn main(){
    let result = is_even(130);
    match result {
        Ok(num)=>{
            println!("Number is even: {}", result);
        }

        Err(msg) =>{
            println!("Error msg: {}", msg);
        }
    }
}

fn is_even(n: i32) -> Result<bool,String>{
    if n%2 ==0{
        return Ok(true);
    }else{
        return Err("NOT_AN_EVEN".to_string());
    }
}
*/


//HANDLE ERROR USING unwarp() //short hand method of above match
/*
fn main() {
    let res = is_even(10).unwrap();
    println!("result is {}", res);
    println!("End of code!");
}
*/

fn main() {
    let res = is_even(19).expect("NOT_AN_EVEN");
    println!("result is {}", res);
    println!("End of code!");
}




fn is_even(n: i32) -> Result<bool,String>{
    if n%2 ==0{
        return Ok(true);
    }else{
        return Err("NOT_AN_EVEN".to_string());
    }
}