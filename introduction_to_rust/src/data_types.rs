use std::{num::ParseIntError, path::PrefixComponent};

pub fn data_types_fun(){
    println!("Data Types");

    //Scalar Type - Holds a single vaule 
    // integer, float, Boolean, Character

    // Integer 
    // 1. Signed - i8, i16, i32, i64, i128

    let account_balane: i32 = -1500;

    // 2. UnSigned - u8, u16, u32, u64, u128
    let number_of_players: u32 = 15;

    // float - Decimal numners
    let overs: f32 = 6.4;

    // Precision - Number of decimal points
    // f32 - 6 to 7 digits
    // f64 - 15 - 17 digits 

    let money_debit: f32 = 345.6454;

    //usize & isize - Detects the bit of operating system / architecture (32bit/64bit)
    let auto_usize: usize = 32;

    let auto_isize: isize = -4534;

    // STRINGS

    println!("String Litral");
    println!("Hello \tescape \tsequence");

    // Raw String - This treat entire string as litrally string 
    // ignoring special characters

    println!(r"C:\Windows\System32\drivers\etc\host");

    //Methods

    println!("Abs method account_balane.abs() - {}", account_balane.abs());
    let empty_space = "       sample Text       ";
    println!("Removing the space around the string - {}", empty_space.trim());

    println!("{number_of_players} Power of 2 - {}", number_of_players.pow(2));


    // Format Specifier
    println!("{money_debit} with {{money_debit:.2}} format specifier - {money_debit:.2} ");
    println!("Can also be mentioned like {{:.2}} - {:.2}", money_debit);

    // type casting

    let value:i32 = 234;
    let value_i64: i64 = value as i64;

    let money_debit_int: i64 = money_debit as i64;
    println!("{money_debit} as {money_debit_int}");


    // Arithmetic operations

    let a: i32 = 13;
    let b: i32 = 3;

    println!("Addtion {a} + {b} = {}", a + b);
    println!("Subtraction {a} - {b} = {}", a - b);
    println!("Multiplication {a} * {b} = {}", a * b);
    println!("Floor Divison {a} / {b} = {}", a / b);
    println!("Modulous[Remainder] {a} % {b} = {}", a % b);

    let a_float: f32 = a as f32;
    let b_float: f32 = b as f32;
    println!("Float Division {a_float} / {b_float} = {}", a_float / b_float);

    // Augumented assignment operator

    let mut year = 2025;
    year = year + 1; // year = 2026 

    //alternative syntax 

    year += 1; // year = 2027

    // This works with - * etc


    // Boolean Type

    let is_sundar_handsome: bool = true;

    let age = 19;
    let is_allowed_to_vote: bool = age > 18;

    println!("{age} - {{is_allowed_to_vote}} - {}", is_allowed_to_vote);

    // Boolean Inversion

    let is_not_allowed_to_vote: bool = !is_allowed_to_vote;
    println!("{age} - {{is_not_allowed_to_vote}} - {}", is_not_allowed_to_vote);


    // Eqality and Inequality operators
    // Equals == 
    // Not Equals !=

    println!("{}", age == 18);
    println!("{}", age != 18);

    // And Logic &&

    let having_id: bool = true;

    let can_vote: bool = age >= 18 && having_id;
    println!("Return true if both condtions are true {{can_vote}} = {{age}} >= 18 && {{having_id}} - {can_vote}");

    // Or Logic ||

    let user: &str = "admin";
    let has_subscription: bool = false;
    let can_view_content: bool = user == "admin" || has_subscription;

    println!("Return true even one condition is true - can_view_content - {}", can_view_content);

    // Character type

    let initial = 'B';
    let emoji: char = '\u{1f980}';
    println!("I love rust - {emoji}");

    // Array - Compound type of data 
    // Fixed size - same type 

    let numbers: [i32; 4] = [48, 66, 33, 54];

    // accessing the array element

    println!("{}", numbers[0]);
    println!("{}", numbers[1]);
    println!("{}", numbers[2]);
    println!("{}", numbers[3]);


    // let numbers_different_type: [i32; 3] = [23.0, 23, 45, 34]; // Throw error as 23.0 is float but our array is i32

    // writing the array elements
    // we need to have mutable array

    let mut names: [&str; 3] = ["Sundar", "Presilla", "Sarvini"];
    println!("{}", names[2]);
    names[2] = "Pappu";
    println!("{}", names[2]);
    
    // Display trait
    println!("{}", names[2]); // {} - this interpolation is display trait 
    // Not all the types have the display trait 

    // Debug trait
    println!("{:?}", names); // print the full array
    println!("{:#?}", names); // pretty print the array
    println!("{names:?}"); // Can also be written like this


    // Debug Macro - Debug Macro
    dbg!(2 + 2); // Debug macro is helpful to have complete code from where it run
    // It is helpful for developers

    // example 
    dbg!("{}", numbers[0]);

    // Tuple - Fixed size, Different data type

    // let employee:(&'static str, i32, &'static str, f64) = ("Sundar", 32, "Engineer", 2.6);
    let mut employee: (&'static str, i32, &'static str, f64) = ("sundar", 32, "Engineer", 2.9);

    // Accessing Tuple index 

    println!("{} - {} - {}", employee.0, employee.1, employee.2);

    // let name = employee.0;
    // let age = employee.1;
    // let role = employee.2;

    // or we can use like below

    let (name, age, role, experince) = employee;

    println!("{} - {} - {} - {}", name, age, role, experince);

    employee.3 = 3.0;

    println!("{:#?}", employee);


    // Ranges - Sequence of elements

    let month_days = 1..31; // Excluding 31
    println!("{:?}", month_days);

    let month_days = 1..=31;
    println!("{:?}", month_days);


    for days in month_days {
        println!("{}", days);
    }

    let alpha = 'a'..'n';
    for alphabet in alpha{
        println!("{alphabet}");
    }


    // Generic

    let numbers: std::ops::Range<i32> = 1..5;
    println!("{numbers:?}");


    let test = 245;
}