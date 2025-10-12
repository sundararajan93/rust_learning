fn main() {
    
    // If Statement executes when the condition is true

    if true {
        println!("The code will execute");
    }

    if false {
        println!("This code will not execute");
    }

    let number = 37;

    if number % 2 == 0 {
        println!("EVEN");
    }

    if number % 2 != 0 {
        println!("ODD");
    }


    // else if --> the if condition is not met we can have multiple 'else if' to control the flow

    let season = "dfgdg";

    if season == "Summer" {
        println!("Need Airconditioners!");
    } else if season == "Winter" {
        println!("Soo Cold");
    } else if season == "Fall" {
        println!("Leaves are falling down");
    } else {
        println!("Not sure what season it is");
    }
    
    let age: i32 = 17;

    let can_vote = if age >= 18 {
        true
    } else {
        false
    };

    let can_vote = if age >= 18 { true } else { false }; // Can be writter like this too
    // when we assign if statement value in variable there should be type consistency

    println!("can_vote - {can_vote}");

    // match statement

    let evaluation: bool = true; // We using boolean for match 

    match evaluation {
        true => {
            println!("evaluation is True");
        } // For true case
        false => {
            println!("evaluation is False");
        } // For false case
    } // We need to specify both the cases as evaluation have two possible values


    // we can also assign the match statement to a variable like below
    // and the possibilites can return value directly
    // during the return even {} curly braces are not required
    let value = match evaluation {
        true => 20,
        false => 49
    };

    println!("{value}");

    // Match statement above have only two values to match but what if we choose string 
    
    let season: &str = "spring";

    match season {
        "summer" => {
            println!("Soo Hot summer");
        },
        "winter" => {
            println!("Soo Cold");
        },
        "spring" => {
            println!("Get my umbrella it spring");
        }
        "fall" => {
            println!("Leaves are falling");
        }
        _ => {
            println!("Unknon season");
        }
    }

    // Matching the multiple values in match arm

    let number: i32 = 9;

    match number {
        2 | 4 | 6 | 8 => {
            println!("{number} is even");
        }
        1 | 3 | 5 | 7 | 9 => {
            println!("{number} is odd");
        }
        _ => {
            println!("Not sure about the number");
        }
    }


    match number { 
        value if value % 2 == 0 => println!("even"),
        valueb if valueb % 2 == 1 => println!("odd"),
        _ => unreachable!()
    }

    // Option with OR pattern - Realtime scenario

    let status_code = 404;

    match status_code {
        200 | 201 | 202 => println!("{status_code} Success!"),
        400 | 401 | 403 => println!("{status_code} Client Error"),
        500 | 502 | 503 => println!("{status_code} Server Error"),
        404 => println!("{status_code} Not Found"),
        _ => println!("{status_code} Unknown status code!"),
    }

    // println!("{status_code}");

    // Range Patterns - The Matching arm will grouped based on the Range

    let mark = 83;

    match mark {
        0..=39 => println!("Fail!"),
        40..=59 => println!("Pass!"),
        60..=79 => println!("Average"),
        80..=100 => println!("Distinction"),
        _ => println!("Unknown Mark"),
    }


    let mut attempts = 3;
    let mut pin = 1234;

    loop {
        if attempts <=0 {
            println!("Exhausted all Attempts! Try again tomorrow!");
            break; // breaks the loop and comes out
        }

        attempts -= 1;

        println!("Remaining Attempt -  {attempts}");
    }

    // continue statement

    let mut number: i32 = 21;

    loop {
        if number <= 0 {
            println!("Loop ends here with break - {number}");
            break;
        }

        number -= 1;
        if number % 2 == 0 {
            println!("Even number found {number}... continues");
            continue;
        }

        println!("Loop - {number}");
    }

    // A While loop continues iterating as long as a condition is met
    // It doesn't need break keyword to end the loop
    // If the condition is false it would exit out of loop automatically

    let mut number: i32 = 21;
    while number > 0 {
        println!("number is {number}");
        number -= 1;
    }

    println!("Time is up!!!");


    // Recursion - calling a function within a function
    // lets say we need to have a countdown

    // let countdown = 10;
    fn countdown(seconds: i32) {

        if seconds == 0 {
            println!("Countdown limit reached");
        }else {
            println!("{seconds} seconds more...");
            countdown(seconds - 1 );
        }
    }

    countdown(5);

}
