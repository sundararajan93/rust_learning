
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

}

#[derive(Debug)]
struct MyStructExample {
}