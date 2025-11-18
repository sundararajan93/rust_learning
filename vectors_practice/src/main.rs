fn main() {
    // Vectors - Similar to arrya but Subsequent homogeneus data (concequent same type of date) which can grow shrink
    // Difference between array and vector is, we cannot add or remvoe element in array
    // But in vector we can make it grow by adding new element and we can remove existing element

    // Creating a empty vector with new() method
    let pizza_diameter: Vec<i32> = Vec::new();

    // Specify the type of Vector with turbofish operator
    let pizza_diameter = Vec::<i32>::new(); // Here we don't need to explicitly annotate the type

    // Vector doesn't use display trait. It uses debug trait
    // printing the vector
    println!("{pizza_diameter:?}");

    // If we know the values already we could directly create vector with vec! macro
    let mut pizza_diameter = vec![6, 8, 10];
    println!("{pizza_diameter:?}");

    // Adding and Removing elements in Vector

    // push --> Adding element to the last 
    pizza_diameter.push(14);
    println!("{pizza_diameter:?}");

    // pop --> Remove the last element and return
    let last_removed = pizza_diameter.pop();
    println!("{last_removed:?} - Has been removed");
    println!("{:?}", pizza_diameter);

    // insert --> Insert in any index 
    pizza_diameter.insert(0, 4);
    println!("{pizza_diameter:?}");

    // remove --> Remove any index element
    pizza_diameter.remove(3); // This create panic if the index goes out of bound
    println!("{pizza_diameter:?}"); 

    // accessing the vector

    let six_inches = pizza_diameter[1];
    println!("{six_inches}");

    println!("{pizza_diameter:?}"); // Value is accessible as it lives in stack and we have copy trait

    let toppings = vec![
        String::from("Mushroom"),
        String::from("Paneer"),
        String::from("Bacon"),
    ];
    // let bacon_toppings = toppings[2]; // This throw error as the move occurs because value has type `String`, which does not implement the `Copy` trait
    let bacon_toppings = & toppings[2]; // So we consider using borrow operator to reference
    println!("{bacon_toppings}"); 

    // slicing with string slice

    let pizza_slices = [1,2,3,4,5];
    let first_slice = &pizza_slices[1..4];
    println!("{first_slice:?}");


    let string_slice = &toppings[1..];
    println!("{string_slice:?}");

}
