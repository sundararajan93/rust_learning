use std::io;

fn copy_trait(){
    let time = 2025;
    let year = time;

    println!("time - {time}\nyear - {year}");
}

fn test1(name: &String) {
    println!("{name}");
}

fn main() {

    let age: i32 = 32;
    let is_healthy: bool = true;

    {
        let is_wealthy: bool = false;
    } // is_wealthy goes out of scope here and the memory is cleared 

    // stack 

    /*
    Top of the stack

        | false | --> is_wealthy
        | true  | --> is_healthy
        | 32    | --> age
        |_______|
    
    bottom of stack

    */
    copy_trait();

    let name: &str = "sundar"; // This is neither stack nor heap as the value is preknown during compile time rust embed this value in binary
    let mut department: String = String::new(); // This value is not known during the compile time and thus it is stored in heap    

    let candy: String = String::from("KitKat"); // The KitKat Somehow stored in the binary executable however 
    // the candy is heap as it fetches space in heap during runtime


    println!("{name} - {department} - {candy}");

    // push_str method 

    department.push_str("Cyber");
    println!("{name} - {department} - {candy}");

    println!("{}", department.capacity());


    let message = "Hey there";
    

    let message: &str = "hey there";
    
    let mut message: String = String::from("Hi there");
    println!("{}", message.capacity());

    message.push_str("i'm growing");
    println!("{}", message.capacity());

    
    // // Getting input from user - Snippet 
    // println!("Enter your name: ");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input);
    // println!("Hello, {name}");
    // // We shall revisit this later


    let sample: String = String::from("Sample Value");
    println!("sample - {sample}");

    let sample2 = sample; // This reassignment would move the sample to sample2
    println!("sample2 - {sample2}");

    // println!("sample - {sample}"); // This will not work as the sample is already been moved from the ownership and scope
    // we cant have multiple owner 

    
    // drop function - Drops the variable from memory explicitly (works only with heap)
    let test: String = String::from("test Value");
    println!("test - {test}");
    drop(test); // Explicitly drop the value
    // println!("test - {test}"); // test would be dropped explicitly so there would be error


    // clone - In some cases we need to copy the variable value we can do with clone method

    let person: String = String::from("Sundar");
    let genius: String = person.clone();
    println!("{person}"); // Note that this clone is going to be expensive as another heap is going to be created.


    let my_stack_value = 45;
    let my_stack_value_ref = &my_stack_value;

    println!("{my_stack_value} - {my_stack_value_ref}");

    let my_heap_value: String = String::from("Baleno");
    let my_heap_reference = &my_heap_value;

    println!("{my_heap_value} - {my_heap_reference}"); // The heap reference holds the value of my_heap_value


    // Reference example with function 
    // Since we reference the text in test1 function the scope still valid within main() if we directly refer text the ownership would changed to test1 fucntion and ends when function ends 
    // Thus makes the text variable unusable
    let text: String = String::from("Sundar");
    test1(&text);
    println!("{text}");


    // Dereference - Retrieving the value that is refererred 
    let who_am_i: String = String::from("Rustacean");
    let who_am_i_ref = &who_am_i;
    println!("Reference - {}, Dereference - {}", who_am_i_ref, *who_am_i_ref); // Dereference can be done only with the reference which points to data in a address

    // Practical use of dereference
    let mut value = 9;
    let value_ref = &mut value;
    println!("{value_ref}");
    // value_ref += 1; // We cant do this as value_ref is just pointing to address (println! macro just helps to derefer automatically so we must use dereference operator explicitly)
    *value_ref += 1;
    println!("{value_ref}");


    // Dereference - practical use in string 
    let mut who_am_i: String = String::from("Rustacean");
    let who_am_i_ref = &mut who_am_i;
    who_am_i_ref.push_str("s");
    println!("Reference - {}, Dereference - {}", who_am_i_ref, *who_am_i_ref); // Dereference can be done only with the reference which points to data in a address


    // Copy vs Move (&str vs String)
    let original_text = "string literal";
    let copy_text = original_text;
    println!("{original_text} - {copy_text}");


    let original : String = String::from("Some String");
    let copy = original;
    // println!("{original} - {copy}"); // original will not be owner as ownership changed already

    // Function Parameters - Function parameters are similar to variable
    // STACK FUNCTION OWNERSHIP
    let no_of_apples: i32 = 9;
    print_no_of_apples(value); // Since its stack value value 9 is stored in two places (value and no_of_apples)
    // value goes out of scope as the function ends
    println!("no_of_apples - {no_of_apples} is valid after the function invocation");


    // HEAP FUNCTION OWNERSHIP
    let whoami: String = String::from("root");
    greet_user(whoami); // let name = whoami --> Moving the variable ownership from whoami to name
    // println!("{whoami} is invalid here as the ownership is trasferred to name parameter of function");


    // Mutable parameters
    let pizza: String = String::from("Pizza");
    add_toppings(pizza); // This passes the pizza to function and the toppings will be pushed to pizza value
    // println!("{pizza}") // Pizza is invalid as the ownership is moved

    // return function --> assinging the function to a variable --> This transfers the ownership from function bake_cake to variable cake
    let cake: String = bake_cake();
    println!("{cake} cake");
    // the variable cake is valid through out the scope of main block
}

fn print_no_of_apples(value: i32) {
    println!("I have {value} apples");
}

fn greet_user(user: String) {
    println!("Hello {user}");
}

fn add_toppings(mut toppings: String) {
    toppings.push_str(" Cheese and Corn with Paneer"); // If the toppings are not mutable this wouldn't push the string to string parameter
    println!("{toppings}");
}

// Return value

fn bake_cake() -> String {
    String::from("Cheese")
}