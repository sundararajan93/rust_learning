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
}
