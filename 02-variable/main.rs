fn main() {
    println!("Hello, world!");
    variable();
    casting();
}


fn variable() {
    let _x:  i8 = 5; // declare a variable `x` of type `i32
    let x:  i64 = 23444;
    println!("{}", x);

    // let mut is_true:bool = true;
    // is_true = false;
    // println!("{}", is_true);

    let mystr = 'A';
    println!("{}", mystr);

    let mut first_name = "Dhanush";
    println!("{}", first_name);
    first_name = "kumar";
    println!("{}", first_name);

    // Tuble
    let name = ("Dhnush","kumar",19, 8.14);
    println!("{:?}", name);

    //Array
    let ages:[u32;6] = [23, 56, 12, 89, 45, 30];
    println!("{:?}", ages);

    //slice 
    let new_ages = &ages[1..=4];
    println!("{:?}", new_ages);
}

fn casting() {
    let x = 255.5_f32;
    let y: u16= x as u16 - 10;
    println!("{:?}", y);


}
