fn main() {
    // Option - enum which has two variants
    // Some(T) - T is generic type 
    // None
    let a = Option::Some("Test");
    println!("{:?}", a);
    let a = Option::Some(200);
    println!("Here the Some is i32 value {:?}", a);
    
    // For Some we don't need to specify type but for None we have to specify Type
    let b: Option<&str> = Option::None;
    println!("{:?}", b);

    // We can use turbofish Operator to specify the type explicitly for None or Some(T)

}
