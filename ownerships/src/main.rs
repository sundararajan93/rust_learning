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




}