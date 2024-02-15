/*
fn main(){
    let v = vec![1,2,3,5,7];
    // print_vector(v);//this code are return borrow the value to function
    print_vector(&v);
    println!("vector v first value: {:?}", v[0]);
}

fn print_vector(val:&Vec<i32>){
    println!("Insert value: {:?}", val);
}

*/

//MUTABLE REFERANCE IN INTEGER
/*
fn main() {
    let mut i = 3;
    add_one(&mut i);
    println!("{}", i);
}
fn add_one(e: &mut i32){
    *e +=1;
}
*/

//MUTABLE REFERANCE IN STRING

fn main() {
    let mut name: String = String::from("Dhanush");
    chan_display(&mut name);
    println!("Name after modification: {}", name);
}

fn chan_display(name_new: &mut String) {
    println!("Before modification:{}", name_new);
    name_new.push_str(" Kumar");
} 