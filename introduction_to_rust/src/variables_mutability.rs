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


}