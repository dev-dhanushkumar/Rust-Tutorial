pub fn test_option() -> Option<u8> {
    let mut opt1:Option<u8> = None;
    // To changing the opt1 value could
    // not change directly opt1 = 10
    //  so, Here we are using Some() variant here... 
    // println!("In Option Test Method: {:?}", opt1);
    opt1 = Some(10);
    return  opt1;
}

pub fn test_option_string() -> Option<String>{       
    let mut opt2:Option<String> = None;
    // println!("In option in String: {:?}",opt2);
    opt2 = Some("Dhanush Kumar".to_string());
    opt2
}

pub enum CourseType {
    Python,
    Java,
    Golang,
    Rust
}

pub fn test_option_course() -> Option<CourseType> {
    let mut course_type: Option<CourseType> = None;
    course_type = Some(CourseType::Golang);
    return course_type;
}

impl ToString for CourseType {
    fn to_string(&self) -> String {
        match self {
            CourseType::Python => "python",
            CourseType::Java => "Java",
            CourseType::Golang => "Golang",
            CourseType::Rust => "Rust"
        }.to_string()
    }
}

pub fn test_option_character() -> Option<CharcterType>{
    let character_name: Option<CharcterType> = Some(CharcterType::Alok);
    return  character_name;
}