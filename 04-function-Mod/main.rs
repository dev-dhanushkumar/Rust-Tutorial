pub mod helpers;

fn main() {
    println!("Hello, world!");
    let name = helpers::test_helper::test_fn("Dahnush","Kumar");
    println!("{0}", name);

    let new_age = helpers::collect_age::new_age_plus_5(20);     
    println!("{0}", new_age);
}