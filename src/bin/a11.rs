// Hashmap

use std::collections::HashMap;
fn main() {
    let mut items = HashMap::new();
    items.insert("Chairs",5);
    items.insert("Beds",3);
    items.insert("Tables",2);
    items.insert("Couches",0);
    let mut total:i32=0 ;

    for (item,qauntity) in items.iter() {
     total = total + qauntity;
     match qauntity {
        0 => println!("Out of Stock"),
        _ =>  println!("we have {:?} total number of {:?}",qauntity,item),

     }
    }
    println!("we have {:?} total number of items",total);

}