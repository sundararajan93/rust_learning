fn main() {
    // Slice is reference for the sequence of 
    let language = String::from("RUST LANGUAGE");
    let lang_ref = &language[0..5]; //RUST
    println!("{lang_ref}");

    let lang_ref = &language[5..];
    println!("{lang_ref}");

    // assigning the variable to block and returning the slice reference
    // we are using &str - The value is embedded to binary and it is already a reference &str
    // Thus the below example is not a dangling reference as it is individual copy of the reference
    let v = {
        let action_hero = "Iron man";
        &action_hero[5..8]
    };


    println!("{v}");

    // slice length - Every alphabetic char is of one byte but this excludes for the emojis

    let food = String::from("Pizza");
    let food_len = &food[0..3];
    println!("{} - {}", food_len, food_len.len());

    // But this slice in emoji and length of an emoji differs 
    // Every emoji contains 4 bytes

    let pizza = "🍕";
    // let pizza_ref = &pizza[0..5]; // This gives panic error as byte index is out of range (emoji contains 4 bytes [0..4])
    // println!("{}", pizza_ref);


    // Syntactic shortcuts
    let language = String::from("RUST LANGUAGE");
    println!("{}", &language[5..]); // Starts all over the end of the string

    println!("{}", &language[..5]); // Start from beginning till the range we specify

    println!("{}", &language[..]); // whole string as slice reference (for some reason if required as slice)


    // Array slices

    let marks = [45, 87, 94, 23, 45];



}
