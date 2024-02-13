use std::collections::HashMap;

pub fn test_hashmap() {
    let mut stock_list = HashMap::<String, f32>::new();

    println!("{:?}", stock_list.len());
    println!("{:?}", stock_list.is_empty());


    //Insert the date into the Map
    stock_list.insert("Anomina".to_string(), 674.90);
    stock_list.insert("Mrandaw".to_string(), 823.12);

    //Overwrite the value.
    stock_list.insert("Anomina".to_string(), 900.90);

    println!("{:#?}", stock_list);
    
    //Remove data from the map;
    stock_list.remove(&("Mrandaw".to_string()));

    stock_list.entry("Meta".to_string()).or_insert(845.76);
    //Don't allow above override the date
    stock_list.entry("Meta".to_string()).or_insert(999.76);
    println!("{:#?}", stock_list);

    //Access the hash values using for loop
    for (company, current_stack) in stock_list{
        println!("{} is trading at {}", company,current_stack);
    }


}