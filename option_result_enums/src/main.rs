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
    let b = Option::<i32>::Some(200);
    println!("i32 specified explicitly with Turbofish Operator -  Option::<i32::Some(i32 value) - {:?}", b);


    // Practical use of Option Enum 
    // lets say we have Array 

    let heros_list = [
        String::from("Iron Man"),
        String::from("Captain America"),
        String::from("Hulk")
    ];

    // If we wan to retrieve the elements of arry we should use index position like below

    println!("{}", heros_list[2]);
    
    // But below line gives panic error as there isn't index position 20.
    // println!("{}", heros_list[20]);

    /*  
        error: this operation will panic at runtime
      --> src\main.rs:34:20
       |
    34 |     println!("{}", heros_list[20]);
       |                    ^^^^^^^^^^^^^^ index out of bounds: the length is 3 but the index is 20
       |
       = note: `#[deny(unconditional_panic)]` on by default

    error: could not compile `option_result_enums` (bin "option_result_enums") due to 1 previous error

    This can't be identified during development phase but only while in runtime we will get the error
    
    */
    

    // To address this issue we shall use get method which has enum of Option

    let iron_man = heros_list.get(0);
    // This returns the index position zero 
    // The enum type this get method have is Option and it will give Some variant if it found some value in that index position
    println!("Index Found - {:?}", iron_man);
    println!("expect method {}", iron_man.expect("Unable to retrieve the value"));

    // Whereas if we do the same index out of bound like 20 
    let unknown_hero = heros_list.get(20); 
    // Rust Option enum which is bound to get method would check for None variant 
    // This eliminate panic error and gracefully return None

    // println!("Index not found - {:?}", unknown_hero);
    // println!("expect method {}", unknown_hero.expect("Unable to retrieve the value"));

    // Unwrap the actual value
    let valid_hero = iron_man.unwrap();
    println!("{valid_hero}");

    // Problem with unwrap is it would run into problem when it deals with invalid value 

    // println!("{}", unknown_hero.unwrap()) ; // Panic error

    
    // Match keyword solution
    println!("Using Match with Option Enum ");
    match iron_man {
        Option::Some(hero) => println!("{hero}"),
        Option::None => println!("No hero found"),
    }

    check_hero_exist(iron_man);


    match unknown_hero {
        Option::Some(hero) => println!("{hero}"),
        Option::None => println!("No hero found"),    
    }

    // Prelude - Option is alread loaded so Option::Some can be written as Some directly, 
    // check below which is right code and doesn't have any difference from other implementation of Option
    match unknown_hero {
        Some(hero) => println!("{hero}"),
        None => println!("No hero found"),
    }

    check_hero_exist(unknown_hero);
    // But in this case we require two match statement for checking both valid and invalid syntax

    let users_in_ldap = [
        String::from("sundar"),
        String::from("vinoth"),
        String::from("mani")
    ];

    let users_in_db = [
        String::from("sundar"),
        String::from("vinoth"),
        String::from("mani"),
        String::from("Gowtham")
    ];

    let user_to_check = "sundar";

    let is_exist_in_ldap = users_in_ldap.contains(&user_to_check.to_string());
    let is_exist_in_db = users_in_db.contains(&user_to_check.to_string());
    println!("LDAP - {}\nDB - {}", is_exist_in_ldap, is_exist_in_db);

    let user_status = is_user_synced(is_exist_in_ldap, is_exist_in_db);
    println!("user existence status - {:?}", user_status);

    // unwrap_or
    // similar to unwrap but provides a fall back value. unwrap the value or provide the fallback value
    let value = Some(78);
    let unknown_value: Option<i32> = None;
    println!("{}", value.unwrap_or(0));
    println!("{}", unknown_value.unwrap_or(0));


}   

// Function with match statemetn

fn check_hero_exist(hero: Option<&String>) {
    match hero {
        Option::Some(hero) => println!("Matches - {hero}"),
        Option::None => println!("Not Matching - No hero found"),
    }
       
}

fn is_user_synced(is_exist_in_ldap: bool, is_exist_in_db: bool) -> Option<bool> {

    if is_exist_in_ldap && is_exist_in_db {
        Option::Some(true)
    } else if is_exist_in_db {
        Option::Some(false)
    } else {
        Option::None
    }

}
