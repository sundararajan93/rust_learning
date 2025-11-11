
// Generic - Type of the argument/parameter
// for example
// below function just simply returns the i32 value that is passed
fn identity_i32(value: i32) -> i32 {
    value
}
// What if I need the same function which accepts f64 parameter - I just need to create another function
fn identity_f64(value: f64) -> f64 {
    value
}

// This makes tough time for us to create multiple functions for multiple types
// But generic solves the problem

fn identity<T>(value: T) -> T {
    value
}
// Breakdown of the above code
// Generic is a place holder for the Type
// We specify <T> as a generic type near function name 
// We specify the same T to the type of the value 
// Finally we returns the same T as return type
// This will accept any type of value to be used as value parameter while invoking them
// Note that T is common community convention- we can use any Name there for the generic


//Multiple Generics
// function may have multiple generics
fn make_tuple(first: i32, second: f64) -> (i32, f64) {
    (first, second)
}// This function creates tuple with two elements of i32 first and f64 type as second element

// lets recreate it with Generic of first element
fn make_tuple_with_generic<T>(first: T, second: f64) -> (T, f64) {
    (first, second)
} // This function accepts any type of data in first parameter as it is T generic type. 
// So we specify T whereever we use first parameter also in return 

// Make both the parameters generic type
fn make_tuple_both_generic<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}// We cant specify T for both the genric as rust will not able to allow as T can be string at first and i32 at second parameter
// So we must use the second common community convention letter U. 
// This covers both elements in any type 

fn main() {
    println!("{}", identity(5));
    println!("{}", identity(true));
    println!("{}", identity(String::from("Test Hello")));
    println!("{}", identity(6.890));
    println!("{:?}", identity(MyStructExample {})); // We can use even Struct type when we use generics

    //TurboFish Operator - ::<i32>
    // we know the identity accepts any value but i need i8 type how can I specify
    // using turbofish operator like below we could annotate our desired type as well
    println!("{}", identity::<i8>(4)); 

    // Multiple Generic function call
    let tuple = make_tuple_both_generic(31, String::from("Hello"));
    println!("{:?}", tuple);

    // Lets use the generic to instantiate the variable with Dynamic type value for treasure
    let gold_chest = TreasureChest {
        captain: String::from("Captain Cool"),
        treasure: "Gold", // using &str in generic field
    };
    println!("{gold_chest:#?}");

    let silver_chest = TreasureChest {
        captain: String::from("Captain Jacksparrow"),
        treasure: false, // Using boolean Type in genric field
    };
    println!("{silver_chest:#?}");

    let special_chest = TreasureChest {
        captain: String::from("Bloodline"),
        treasure: ["Gold", "Silver", "Platinum", "Diamonds"], // Using array type of &str in all the elements in place of generic field
    };
    println!("{special_chest:#?}");

}

#[derive(Debug)]
struct MyStructExample {
}

// Generics in struct
// We could use generic in struct for a field which we doesn't know the type in advance

#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}