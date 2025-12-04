mod inventory;

// using both the constructor and the module too. using self keyword
use inventory::item::{self, Item};
// the below is same as above statement
// use inventory::item::Item; 
// use inventory::item;


// using the constructor
// use inventory::item::Item;

// using only till the module
// use inventory::item;

// alias

// use inventory::MANAGER;
// use inventory::item::MANAGER; // This line would throw error as the rust cannot have both MANAGER variable for different module

// Here is were the as keyword from alias comes handy
use inventory::MANAGER as INVENTORY_MANAGER;
use inventory::item::MANAGER as STORE_MANAGER;

fn main() {
    
    // let item = inventory::Item {
    //     name: "pen",
    //     count: 100,
    // };

    println!("{}", inventory::STORE_OWNER);

    let pen = Item {
        name: String::from("PEN"),
        count: 8,
    };

    let paper = item::Item {
        name: String::from("whitepaper"),
        count: 199,
    };

    println!("{:?}", paper);
    println!("{:?}", pen);

    let pencil = Item::new(String::from("pencil"), 50);



    
}
