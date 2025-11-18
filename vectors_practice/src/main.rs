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
    let pizza_diameter = vec![6, 8, 10];
    println!("{pizza_diameter:?}");


}
