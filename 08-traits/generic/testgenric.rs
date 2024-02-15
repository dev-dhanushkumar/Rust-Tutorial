//Data Type Miss Match
/*
fn main(){
    let mut vector_int = vec![34,78,45];
    vector_int.push(45);
    vector_int.push("Hello"); //Error are there in this place different data type
    println!("{:?}", vector_int);

}
*/

struct Data<T> {
    value: T,
}

fn main(){
    //generic type of i32
    let t: Data<i32> = Data{value:89};
    println!("{}", t.value);
    //generic type of String
    let t2: Data<String>  = Data{value:"Dhanush".to_string()};
    println!("{}", t2.value);
}