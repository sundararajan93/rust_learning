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
    
}
