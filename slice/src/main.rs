use std::path::PrefixComponent;

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

    println!("{:?}", &marks[0..3]);
    println!("{:?}", &marks[3..]);
    println!("{:?}", &marks[..4]);
    println!("{:?}", &marks[..]);


    let marks_ref = &marks[2..5]; // This is the slice. if you notice we have &[i32] as type but doesn't give no of elements
    // This is because of the length dynamic nature
    let marks_ref = &marks; // however this is the reference of the entire array so the type is &[i32; 5] with number of elements
    println!("{:?}", &marks_ref[2..5]);

    // Rust does not permit mutable slice of string, however it does support mutable slice of array
    let mut values = [23, 64, 235, 63, 62, 12, 63];

    let my_slice = &mut values[2..5];
    println!("{:?}", my_slice);

    my_slice[0] = 24;
    println!("{:?}", my_slice);


    let mut sample_array = [25, 65, 47];
    sample_array[1] = 82;
    println!("{:?}", sample_array); // In this example we directly mutate the value and replace
    // But with Reference we caould slice and borrow the value to change 

    let sample_array_ref = &mut sample_array[..];
    println!("{:?}", sample_array_ref);
    sample_array_ref[1] = 57;
    println!("{:?}", sample_array_ref);
    
}