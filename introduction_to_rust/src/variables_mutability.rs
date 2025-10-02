const TAX_RATE: f32 = 12.5;

// Type Aliase
type Meters = i32;

pub fn variables_mut() {
    let number_of_apples = 5;
    let number_of_oranges = 10;
    let number_of_fruits = number_of_apples + number_of_oranges;

    println!("I have {} apples.", number_of_apples);
    println!("I have {} oranges.", number_of_oranges);
    println!("I have {} fruits in total.", number_of_fruits);

    println!("I have {number_of_apples} apples {number_of_oranges} oranges, making a total of {number_of_fruits} fruits.");


    let unused_variable = 42; // This will not cause a warning
    // prefix the variable with _
    // Rust knows that the variable is intentionally unused
    let _unused_variable = 43; // This wouldn't throw error even if the the variable is not used

    // Immutable - Rust 

    let gym_reps = 3;
    println!("Gym reps {gym_reps}");

    // gym_reps = 5; // Unable to change after the assignment


    let mut gym_rep_new = 12;
    println!("New Gym Reps - {gym_rep_new}");

    // can be changed
    gym_rep_new = 15;
    println!("New Gym Reps - {gym_rep_new}");

    // Variable Shadowing

    let salary = "10000";
    println!("{salary} - is a string");
    let salary = 10000; // Reinitializing or shadowing same variable with integer
    println!("{salary} - is a integer");
    let salary = 10000.00; // Reinitializing or shadowing same variable with float
    println!("{salary} - is a floating point value");


    // Scope
    let coffee_price = 20.59;
    println!("Coffe Price - Outer Scope Value - {coffee_price}");

    // Block
    {
        // Inner Scope
        println!("Coffe Price variable is valid inside the block / inner scope {coffee_price}");
        let cookie_price = 10;
        println!("Cookie Price is valid only inside the scope - {cookie_price}");

        println!("Constant is valid inside scope {TAX_RATE}");
    }
    println!("Cookie Price is invalid here as it is out of scope");
    println!("Constant is valid outside scope too {TAX_RATE}");

    // Type Aliases
    #[allow(unused_variables)] // compiler directive apply to next line only
    let full_mile_race: i32 = 1600; // Here we have to declare the type as i32. But it isn't meaningful

    let full_mile_race: Meters = 1600; 
    // We created a type alias for i32 with name Meters now everyone know what this variable is


}