enum Access {
    Admin,
    Guest,
    User,
    Manager
}

fn main() {
    let access_file = Access::Admin;
    let result = match  access_file {
        Access::Admin => "You are admin.",
        _ => "You are not access this file!",
    };

    println!("{:?}", result);
}