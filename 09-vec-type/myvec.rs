
#[allow(dead_code)]
pub fn test_vec_int() {
    let mut myint:Vec<i32> = Vec::new();
    myint.push(34);
    myint.push(94);
    myint.push(94);
    myint.push(94);
    myint.push(94);
    //Default capacity is 4 but it will be automatically
    //increase there capacity!
    println!("Size of vec:{:?}", myint.len());
    println!("Capacity of vec:{:?}", myint.capacity());
    println!("{:?}", myint);
    println!("Slice of range type access: {:?}", &(&myint).as_slice()[1..=3]);
    println!("Second Element is: {:?}", myint.get(45));//Out fo index to handle without panic
    println!("Second Element is: {:?}", myint.get(1));
}


#[allow(dead_code)]
pub fn test_vec_string() {
    let first_names = vec!["Dhanush", "austin", "Adhi", "Hitesh"];
    for first_name in first_names.clone(){ //borrow those values usinf slice and clone...
        println!("Processing {} ...", first_name);
    }
    println!("{:?}", first_names);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Dragon{
    damage: String,
    level: u8,
}
pub fn test_vec_dragon(){
    let mut dragon_list:Vec<Dragon> = vec![];
    let mut dragon_list2:Vec<Dragon> = vec![];
    for _ in 1..=50u8{
        dragon_list.push(Dragon{damage:"500 p/s".to_string(), level: 5});
    }
    for _ in 1..=50u8{
        dragon_list2.push(Dragon{damage:"1000 p/s".to_string(), level: 7});
    }

    // println!("{:?}", dragon_list);
    println!("Dragon list length:{:?}", dragon_list.len());
    println!("Dragon list2 length:{:?}", dragon_list2.len());
    println!("{:?}", dragon_list.capacity());

    //Now we are join the two variable using append kerword
    dragon_list.append(&mut dragon_list2);
    //retain mean replace the elements
    let keep = |e: &Dragon| {if e.level == 7 {return true;}else {return false}};
    dragon_list.retain(keep);
    //inset the element to vec
    dragon_list2.insert(0,Dragon{damage:"1500 p/s".to_string(), level: 10});
    println!("Oth index element: {:?}", dragon_list2.get(0).unwrap());
    println!("Dragon list length:{:?}", dragon_list.len());
    println!("Dragon list2 length:{:?}", dragon_list2.len());
    // println!("{:?}", dragon_list);
    dragon_list2.remove(0);
    println!("Dragon list2 capacity:{:?}", dragon_list2.capacity());
    dragon_list2.reserve(100);
    println!("Dragon list2 capacity:{:?}", dragon_list2.capacity());
}