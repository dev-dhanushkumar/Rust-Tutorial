use std::collections::HashSet;

pub fn test_hashset(){
    let mut planet_list  =  HashSet::from(["Mercury","Earth", "Venus"]);

    // for planet in planet_list {
    //     println!("Welcome to {}", planet);
    // }

    let planet_more = HashSet::from(["Mars", "Jupiter", "Earth"]);
    
    //  To different form tow maps
    // let planet_remain = planet_list.difference(&planet_more);

    // for planet1 in planet_remain {
    //     println!("Remaining Planet: {}", planet1);
    // }

    //To insert the values into the hash map
    planet_list.insert("Saturn");
    planet_list.insert("Uranus");

    // let planet_symdiff = planet_list.symmetric_difference(&planet_more);
    // for planet1 in planet_symdiff {
    //     println!("Remaining Planet: {}", planet1);
    // }

    for planet in planet_list{
        println!("Thanks for Adding: {}", planet);
    }
}