fn apply_to_jobs(number: i32, title: &str) {
        println!("I'm applying to {number} {title} jobs");
    }

    fn is_even(number: i32) -> bool {
        number % 2 == 0
    }
    
    fn alphabets(text: &str) -> (bool, bool) {
        let a = text.contains('a');
        let b = text.contains('z');
        (a, b)
        // ( text.contains('a'),  text.contains('b')) // This is a simplified solution too
    }

fn main() {
    apply_to_jobs(35, "Rust Developer");

    is_even(8);
    is_even(9);

    println!("{:?}", alphabets("aardvark")); // (true, false)
    println!("{:?}", alphabets("zoology"));  // (false, true)
    println!("{:?}", alphabets("zebra"));    // (true, true)

    // Terminating the implicit return results in () A unit (empty tuple)
    let mystery = {
        5 + 4;
    };

    println!("{mystery:?}");

    /*
    Define an apply_to_jobs function that accepts a
    'number' parameter (an i32) and a 'title' parameter
    (a string). It should print out the string:
    "I'm applying to {number} {title} jobs".
    
    Example:
    apply_to_jobs(35, "Rust Developer")
    -> "I'm applying to 35 Rust Developer jobs"
    
    Define an is_even function that accepts a 'number'
    parameter (an i32). The function should return a true
    if the number is even and a false if the number is
    odd.
    Examples:
    is_even(8) -> true
    is_even(9) -> false
    
    Define an alphabets function that accepts a 'text'
    parameter (an &str). The function should return a
    tuple of two Booleans. The first Boolean should check
    if the text contains the letter 'a'. The second
    Boolean should check if the text contains the letter
    'z'. You can use the 'contains' method to check if a
    string contains a specific character. See the documentation:
    https://doc.rust-lang.org/std/primitive.str.html#method.contains
    
    Examples:
    println!("{:?}", alphabets("aardvark")); -> (true, false)
    println!("{:?}", alphabets("zoology"));  -> (false, true)
    println!("{:?}", alphabets("zebra"));    -> (true, true)
    */

}


