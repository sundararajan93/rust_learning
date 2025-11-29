use godown::parking::SECURITY_GUARD;

// use godown::{Category, Inventory}; // We can  specify multiple constructs in use statement in single line with this syntax
// Thus this can be easy to call like Category::... and Inventory::... 

// To add module which represented as file
mod store;

// Adding moudle godown which is represented as directory
mod godown;

use godown::Inventory;

// To create module we shall use mod segment
mod inventory {

    pub const MANAGER: &str = "Ivan";

    // let square_feet:i32  = 50005;

    #[derive(Debug)]
    enum ProductCategory {
        Ladder,
        Hammer,
    }

    #[derive(Debug)]
    struct Item {
        name: String,
        category: ProductCategory,
        quantity: u32,
    }

    fn talk_to_sales() {
        println!("{MANAGER} is accessible here inside the module");
    }

}


fn main() {
    // To access the variable or constant from module to anywhere
    // But the module inventory should have the component specified with pub keyword
    println!("{} is the Manager for inventory", inventory::MANAGER);

    // We could have same name of construct in different modules
    println!("{} is the Manager for store", store::MANAGER);

    // We could use the module as folder
    println!("{} is the Manager for godown", godown::MANAGER );

    // Creating struct from the module godown

    // let stock = Inventory {
    //     name: String::from("Ladder"),
    //     quantity: 4,
    //     category: godown::Category::Ladder,
    // };

    // Though we have Inventory struct defined public the fields inside them must be explicitly defined pub to use them
    // Or else by default the name fields are private
    // example
    /*
    pub struct Inventory {
        pub name: String,
        pub quantity: String,
        pub category: godown::Category::Ladder
    
    }
    
    */
    
    let hammer = Inventory::new(
        String::from("10 ft ladder"),
        9, 
        godown::Category::Ladder
    );

    println!("{hammer:#?}");

    println!("{}", godown::parking::SECURITY_GUARD);

    // We can make use of 'use' keyword to shortcut the above usage
    // use godown::parking::SECURITY_GUARD; 
    // this brings the SECURITY_GUART to the scope easily
    println!("{}", SECURITY_GUARD);

    // Note that we shall not use multiple variable in different path with same variable name 


}
