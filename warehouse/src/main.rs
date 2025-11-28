// To add module which represented as file
mod store;

// Adding moudle godown which is represented as directory
mod godown;

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
}
