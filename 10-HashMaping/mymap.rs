use std::collections::HashMap;

pub fn test_hashmap() {
    let mut stock_list = HashMap::<String, f32>::new();

    println!("{:?}", stock_list.len());
    println!("{:?}", stock_list.is_empty());

    stock_list.insert ("Anomina".to_string(), 674.90);
    stock_list.insert("Mrandaw".to_string(), 823.12);

    println!("{:#?}", stock_list);

}