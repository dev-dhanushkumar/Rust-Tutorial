fn main() {
    println!("Hello World!");

    println!("{}", 23);

    println!("{1} and {0} are good friends!", "Hitesh", "kar");

    println!("base 10: {}", 648723);
    println!("base 2 (binary): {:b}", 8746483);
    println!("Base 8 (octal): {:o}", 36462);
    println!("Base 16 (Hexadecimal): {:x}",89665);

    println!("{number:0>5}", number=1);
    println!("{number:0<5}", number=1);
    println!("{number:>5}", number=1);

}