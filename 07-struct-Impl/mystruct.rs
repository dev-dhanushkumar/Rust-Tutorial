use std::cell::Cell;


#[derive(Debug)]
#[allow(dead_code)]
enum  Names {
    Adhi,
    Balaji,
    Austin,
    Dhanush
}

#[derive(Debug)]
#[allow(dead_code)]
struct Student {
    name: Names,
    degree: String,
    age: u16,
    cgba: f32,
    mark: u16,
}

impl Student {
    fn student_name(&mut self, new_name: Names){
        self.name = new_name;
    }
    fn create_student() -> Student{
        let new_student = Student{name:Names::Balaji,degree:"B.E".to_string(),age: 18,cgba:8.4,mark:0};
        return new_student;
    }
    fn addmark(&mut self, marks: u16){
        self.mark += marks;
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct BookTuble (String, String, u16);

fn new_book_tuble() -> BookTuble{
    return BookTuble("Golang".to_string(),"Dhanush".to_string(), 2022);
}

pub fn create_book_tuble(){
    println!("{:?}", new_book_tuble());
}

struct Person<'p>{
    firstname: Cell<&'p str> ,
    lastname: String,
    age: u16,
    birth_year: u16,
}

// fn new_person() -> Person{
//     let mut p1 = Person { firstname: "Dhanush".to_string(), lastname: "Kumar".to_string(), age: 19, birth_year: 2004 };
//     p1.firstname = "Ram".to_string();//Using mut keyword to change all the values        
//     return p1;
// }

//to change only one field from the struct
fn change_one_field() -> Person<'static>{
    let  p2 = Person { firstname: Cell::from("Dhanush"), lastname: "Kumar".to_string(), age: 19, birth_year: 2004 };
    p2.firstname.set("Ram");
    return p2;

}

pub fn test_create_person() {
    let my_person =  change_one_field(); //To assing this variable we could not using public of struct here.
    println!("🦀First Name: {0}, Last Name: {1}, age: {2}, birth_year: {3}", my_person.firstname.get(), my_person.lastname,my_person.age,my_person.birth_year)
}

pub fn test_name_check() {
    let mut student = Student{name:Names::Balaji, degree:"B.E".to_string(), //using enum here for name
    age: 19, cgba: 8.52, mark:0
    };
    student.student_name(Names::Dhanush);
    println!("🦀{:?}", student);
}

pub fn  impl_student_test(){
    let mut my_student  = Student::create_student();
    my_student.student_name(Names::Austin);
    my_student.addmark(60);
    my_student.addmark(40);
    println!("{:?}", my_student);
}