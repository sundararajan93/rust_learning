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


    // mutliple Mutable Reference - In Rust we cant have multiple mutable reference

    let mut program: String = String::from("python");
    let mut_ref1 = &mut program;
    let mut_ref2 = &program; // Throw error if used somewhere 
    let ref3 = &favorite_program;  // Throw error if used somewhere
    // println!("{mut_ref1} {mut_ref2} {ref3}") // This will throw error as the refs are uses
    println!("{mut_ref2}");


    // another example
    let mut car: String = String::from("Red");
    let ref1 = &mut car;
    let ref2 = &car;
    println!("{ref2}"); // If you call mutable reference it will throw error as both write and read happens on same reference



    // Copy trait immutable reference

    let coffee: String = String::from("ColdCofee");
    let a = &coffee;
    let b = a;
    // This is same as
    let b = &coffee;
    println!("both {{a}}- {a} and {{b}} - {b} are independent copy of &cofee (immutable reference)");


    //Copy trait is not implemented in mutable datatypes thus the below code is going to act like ownership movement 
    
    let mut coffee: String = String::from("ColdCofee"); // Mutable variable
    let a = &mut coffee; // mutable reference assigned to variable a (ownership a)
    let b = a;  // Now since the variable a assigned to variable b. now the ownership moved to variable b
    // println!("{a} and {b}"); // In this reference since the ownership moved to variable b from a, a variable gets invalidated 
    // also we know that we should have only one mutable reference
    println!("{{b}} - {b} can be used as its new owner, rust would igore variable a as it was not used anywhere else");


    // we can make immutable reference for the mutable variable too
    let mut example: String = String::from("Testing ");
    let a = &example;
    let b = a;
    println!("a - {a} and {b}");

    // Reference with Array - Stack Datatype

    let config = [true, false, false];
    let first: bool = config[0];
    println!("{{config}} -  {config:?} and {{first}} - {first}");

    // Reference with Array - Heap based Datatype

    let languages: [String; 2] = [String::from("Rust"), String::from("Python")];
    // let first = languages[0]; // We cant assign element to the variable 
    println!("{languages:?} and {first}");
    // This is because the heap data doesn't support copy trait. 

    // Solution to this is to clone() or refer the element with & 
    let first = languages[0].clone();
    println!("{languages:?} and {first}");
    
    // using reference

    let first = &languages[0];
    println!("{languages:?} and {first} with reference to the element");


    // Reference with tuple - Stack Datatype

    let config: (bool, bool, bool) = (false, true, false);
    let first = config.0;
    println!("{config:?} - {first}");


    let fav_color: (String, String) = (String::from("Yellow"), String::from("Black"));
    let first = &fav_color.0;
    println!("{fav_color:?} - {first}");

}



// Four ways you can call the String as parameter in function
// meal: String (regualar Parameter)
// mut meal: String (Mutable regular parameter)
// meal: &String (Immutable reference String parameter) --> Can get the string reference but unable to modify it
// meal: &mut String (mutable reference String parameter) --> Can get the string reference and able to modify it
