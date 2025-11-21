use std::option;

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

    // get method 

    let option = toppings.get(60);
    match option {
        Some(option) => println!("{option}"),
        None => println!("No Toppings"),
    }


    let mut delicious_toppings = toppings; // The ownership moved to delicious_toppings from toppings
    //whenever we try to call toppings it would fail 

    // println!("{toppings:?}"); // This would fail as the ownership is moved

    // creating reference for vector

    let toppings_reference = &delicious_toppings[1];

    println!("{delicious_toppings:?}"); // This works as we create only immutable reference for delecious_toppings

    // Lets make modification to delicious toppings
    delicious_toppings.push(String::from("pepporoni"));

    println!("{delicious_toppings:?}");
    // println!("{toppings_reference:?}"); // This wouldn't work as we have both mutable and immutable reference for a vector same time
    // If we create mutable reference it should be the last one. If we create any number of immutable reference that ends when we make one mutable change
    

    // create a vector
    let mut name_list = vec![String::from("Sundar"), String::from("Parthi"), String::from("Dhivi")];

    // Replacing value with multiple immutabl reference
    let another_list = &name_list[2];
    let name_list_ref = &another_list;

    // With capcity
    let mut product_list:Vec<&str> = Vec::with_capacity(4);
    println!("Length - {} Capacity - {}", product_list.len(), product_list.capacity());
    product_list.push("Product1");
    product_list.push("Product2");
    product_list.push("Product3");
    product_list.push("Product4");
    println!("Length - {} Capacity - {}", product_list.len(), product_list.capacity());

    product_list.push("Product5");
    println!("Length - {} Capacity - {}", product_list.len(), product_list.capacity());


}
