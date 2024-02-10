fn main(){
    test_clouser();
}
struct  Person {
    first_name: String,
    last_name: String
}


pub fn test_clouser(){
    let add = |a, b| {
        let c = a + b;
        c
    };
    let mut res  = add(5,7);
    

    let mut print_result = |x|{
        res = 20;
        println!("Result is: {}", res+x);
    };
    print_result(40);

    let mut p1 = Person{first_name:"Dhanush".to_string(),
        last_name: "kumar".to_string()};
    println!("{0} {1}",p1.first_name, p1.last_name);

    let mut  change_name = |new_name:&str| p1.last_name = new_name.to_string() ;
    
    change_name("Ram");
    println!("{0} {1}",p1.first_name, p1.last_name);
}