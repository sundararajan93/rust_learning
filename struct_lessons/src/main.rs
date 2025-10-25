fn main() {

// Struct is like a blueprint of a type. We have only the details spec and type of fields not the concrete values 
// example below is the namefield for Coffee type
// Here Coffee is something like f64, i32 (type) 
    struct Coffee{
        price: f64,
        name: String,
        is_hot: bool,
    }

    // Creating an instance with Struct
    // Creating an instance should be like creating actual concrete values for type Coffee
    // We can name it and define like below 
    // the namefields or elements can be called in any order. 
    // But it should have all the elements we defined in struct 
    // The value should be of same type which we defined

    let cold_coffee = Coffee {
        name: String::from("Cold Cofee"),
        price: 4.89,
        is_hot: true,
    };

    // Accessing the fields in struct
    // struct.field  is the syntax and we cant use it inside {} interpolation
    println!("{} costs {} and it is {} it is hot", cold_coffee.name, cold_coffee.price, cold_coffee.is_hot);


    // The ownership in struct works hierarchial way
    // cold_coffee variable owns the Coffee Struct
    //    |__ Coffee Struct owns the fields in it
    //            |__ Fields like name, price and is_hot owns the values of them

    // Similar to other data types if the Data is heap based the copy trait wont happen
    // only move trait for String

    let fav_coffee = cold_coffee.name;
    println!("{fav_coffee}");

    // println!("{}", cold_coffee.name);    // It fails here as the ownership moved from cold_coffee.name to fav_coffee
    // So we need to be mindful what we do in using accessing strcut
    // There isn't a problem with stack based types. as that implements copy traits

}
