// Immutable reference - The parameter is referrenced but it is just immutable
// You are borrowing your friend book to read it. Your friend is still owner of the book you need to return it to him

// Here we are creating immutable reference where we specify &String for the datatype meal (reference the String parameter passed)
fn show_dish(meal: &String) {
    println!("{meal}");
}

// Mutable reference - The parameter is mutable and referenced 
// you are borrowing your friend pen to write something

// here we are creating mutable reference for String so we specify &mut also while calling the function you need to pass the parameter with & to reference
fn add_mushroom(toppings: &mut String) {
    toppings.push_str(" Mushroom");
}

fn main() {
    let mut my_meal: String = String::from("pizza");
    show_dish(&my_meal); // Adding my_meal as reference 
    println!("{{my_meal}} - {my_meal} is still valid");

    // mutable reference
    add_mushroom(&mut my_meal);
    println!("{my_meal} is still valid with topping");

    println!("{my_meal} is still valid but with new value");

    // Multiple references 
    // what happens when we require multiple references

    // Multiple immutable reference - Rust allow any number of multiple reference for immutable variable, parameter

    let favorite_program: String = String::from("Rust");
    let reference_1 = &favorite_program;
    let reference_2 = &favorite_program;
    let reference_3 = &favorite_program;

    println!("{} - {} - {} - {}", reference_1, reference_2, reference_3, &favorite_program );

}

// Four ways you can call the String as parameter in function
// meal: String (regualar Parameter)
// mut meal: String (Mutable regular parameter)
// meal: &String (Immutable reference String parameter) --> Can get the string reference but unable to modify it
// meal: &mut String (mutable reference String parameter) --> Can get the string reference and able to modify it
