use mystruct::{test_create_person, test_name_check, create_book_tuble};

use crate::mystruct::impl_student_test;

pub mod mystruct;


fn main() {
    // println!("Hello, world!");
    // let my_person = mystruct::new_person();
    // println!("
    //     First Name: {0}
    //     Last Name: {1}
    //     Age: {2}
    //     Birth_Year{3}",
    //     my_person.firstname,
    //     my_person.lastname,
    //     my_person.age,
    //     my_person.birth_year
    // );
    // println!("Last Name: {0}", my_person.lastname);
    // println!("Age: {0}", my_person.age);
    // println!("Birth Year: {0}", my_person.birth_year);

    test_create_person();
    test_name_check();
    create_book_tuble();
    impl_student_test();
}