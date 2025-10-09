mod variables_mutability;
mod data_types;

const TAX_RATE: f32 = 18.5;
fn main() {
    println!("Hello, world!");

    variables_mutability::variables_mut();

    println!("{TAX_RATE}");

    data_types::data_types_fun();

    // Invoking function

    create_pizza();
    delivering_pizza(); 
    delivering_pizza(); // Reusing the Funtion
    delivering_pizza(); // Reusing the Funtion
    making_profit();
    making_profit();  // Reusing the Funtion
    making_profit(); // Reusing the Funtion
}

// Pizza business
fn create_pizza() {
    println!("Baking Pizzas");
}

fn delivering_pizza() {
    println!("Delivered Pizza");
}

fn making_profit() {
    println!("Making more profit");
}