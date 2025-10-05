use std::path::PrefixComponent;

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


}