pub mod optiontest;

fn main() {
    println!("Hello, world!");
    let res = optiontest::test_option();
    println!("{0}", res.unwrap());

    //test String
    let strresult = optiontest::test_option_string();
    println!("{0}", strresult.unwrap());

    let course_res = optiontest::test_option_course();
    println!("You Selected course Name is : {0}", course_res.unwrap().to_string());

    let character_res = optiontest::test_option_character();
    println!("You selected Character name is: {0}", character_res.unwrap().to_string() );
}