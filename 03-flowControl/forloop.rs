pub fn for_loop() {

    println!("For construct using range!");
    for n in 20..25 {
        println!("for loop curent value: {0}", n);
    }

    println!("\n");

    for n in 20..=25 { // To take the last value of range!
        println!("for loop curent value: {0}", n);
    }
     
    println!("\n");

    println!("foor loop iterators!");
    let names:[&str;4] = ["adhi", "austin", "balaji", "dhanush"];
    for name in names.iter() {
        println!("Name of student: {}", name);
    }
    println!("{:?}", names);

    println!("\n ");

    println!("foor loop in iter_into type");
    let student_list = vec!["adhi", "austin", "balaji", "dhanush"];
    for n in student_list.into_iter() {
        println!("Name of student: {}", n);
    }//after the loop execution student_list will be not available.

    // println!("{:?}", student_list);

    println!("\n");

    println!("To see inter_mut now!");
    let mut reg_no = vec![09,10,05,15,22,23];

    for n in reg_no.iter_mut() {
        *n = match n {
            &mut 15 => 14,
            _ => 10,
        }
    }
    println!("Register Numbers are: {:?}", reg_no);

}