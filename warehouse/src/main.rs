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
        println!("{MANAGER} is accessible here inside the module")
    }

}

fn main() {
    // To access the variable or constant from module to anywhere
    println!("{} Manager for inventory", inventory::Manager);
}
