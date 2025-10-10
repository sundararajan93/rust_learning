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

    // Invoking function with parameters
    add(4,2);


    let square_of_32 = square(32);
    println!("{}", square_of_32);

    let cube_of_32 = cube(32);
    println!("{}", cube_of_32);
    
    let null_fun = null_function(); // This is empty function so the return type is detected as () tuple by default

    // Block without function

    let calculation: i32 = {
        let value = 5 + 4;
        value * 2
    };

    println!("{calculation}");

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


// Functions with parameters

fn add(num1: i32, num2: i32) {
    println!("adding {num1} + {num2} = {}", num1 + num2);
}



fn square(num: i32) -> i32 {
    return num * num;
    println!("This line will not be executed as it is after return statemet");
}

fn cube(num: i32) -> i32 {
    num * num * num
    // This is also a valid function with implicit return. Note this implicit return should be last line and there shouldn't be any semicolon at the end
}

// Unit of function is a tuple by default

fn null_function(){

}