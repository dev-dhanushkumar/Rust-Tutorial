pub fn label_loop() {
    'outer: loop {
        println!("Entered the outter loop here!");

        'inner: loop{
            println!("Entered the inner loop now!");
            break 'outer;
        }
        println!("This point will never reached!");
    }
    println!("Exited outer loop!");
}