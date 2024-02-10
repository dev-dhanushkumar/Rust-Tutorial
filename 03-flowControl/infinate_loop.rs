pub fn infiloop() {
    println!("Let's count the infinite loop here!");
    let mut count = 0;
    loop{
        count +=1;
        if count == 6 {
            println!("Six!");
            continue;
        }
        println!("{}", count);

        if count == 10 {
            println!("loop reach 10 that's enough now! ");
            break;
        }
    }
}