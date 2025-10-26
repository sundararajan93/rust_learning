// Module level struct
#[derive(Debug)]
struct Car {
    name: String,
    color: String,
    price: f64,
}

// Struct for Employee

struct Employee {
    name: String,
    id: i128,
    role: String,
    is_in_notice_period: bool,
}




fn make_my_car(name: String, color: String, price: f64) -> Car {
    Car {
        name: name,
        color: color,
        price: price,
    } // Implicit return of Car struct 
}

fn create_employee(name: String, id: i128, role: String, is_in_notice_period: bool) -> Employee {
    Employee { 
        name: name,
        id: id, 
        role: role, 
        is_in_notice_period: is_in_notice_period 
    }
}

// Struct for Team
struct Team {
    name: String,
    number_of_members: i32,
    is_technical: bool,
}

// In the below function we are going to use the parameters with same name as mentioned in struct Team
// So while returning the struct we don't need to mention the field and parameter as they are same
// Instead of name: name (i.e) <field_name>:<parameter>, we can use only once <same field / param name> 
fn create_team(name: String, number_of_members: i32, is_technical: bool) -> Team {
    Team {
        name, 
        number_of_members,
        is_technical
    }
}

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

    let my_coffee = Coffee {
        name: String::from("Chocolate Cofee"),
        price: 4.89,
        is_hot: true,
    };

    // Accessing the fields in struct
    // struct.field  is the syntax and we cant use it inside {} interpolation
    println!("{} costs {} and it is {} it is hot", my_coffee.name, my_coffee.price, my_coffee.is_hot);

    // The ownership in struct works hierarchial way
    // my_coffee variable owns the Coffee Struct
    //    |__ Coffee Struct owns the fields in it
    //            |__ Fields like name, price and is_hot owns the values of them

    // Similar to other data types if the Data is heap based the copy trait wont happen
    // only move trait for String

    let fav_coffee = my_coffee.name;
    println!("{fav_coffee}");

    // println!("{}", my_coffee.name);    // It fails here as the ownership moved from my_coffee.name to fav_coffee
    // So we need to be mindful what we do in using accessing strcut
    // There isn't a problem with stack based types. as that implements copy traits

    // Note that we can also use reference borrowing like in other data types
    let price_ref = &my_coffee.price; // Only referring the ownership
    println!("{}", price_ref); 
    println!("{}", my_coffee.price); // Ownership preserved to the my_coffee not changed
    // So both works


    // overwrite struct field

    let mut beverage = Coffee {
        name: String::from("Cold Coffee"),
        price: 9.87,
        is_hot: false,
    };

    println!("{} is {} price. And it is {} that it should be hot", beverage.name, beverage.price, beverage.is_hot);

    beverage.name = String::from("Hot Coffe");
    beverage.is_hot = true;

    println!("{} is {} price. And it is {} that it should be hot", beverage.name, beverage.price, beverage.is_hot);
    
    // In the above example we can overwrite the values of the fields we want
    // Condition is that it should be mutable variable 
    // We cant make the struct itself mutable we can only make the instance mutable
    // Note that we cant mutate only one value you have to make the entire fields mutable. Its like all or none


    let ferrari: Car = make_my_car(String::from("ferrari"), String::from("Red"), 8.98);
    println!("I have {} {} which is {} crores", ferrari.color, ferrari.name, ferrari.price);



    // Employee Creation
    let name: String = String::from("Sundararajan Thangaraj");
    let id: i128 = 10560201551;
    let role = String::from("Development Engineer 4");
    let is_in_notice_period = false;

    let employee_1 = create_employee(name, id, role, is_in_notice_period);
    println!("{} with id - {} is working as {}. It is {} that he doesn't serve notice period", employee_1.name, employee_1.id, employee_1.role, employee_1.is_in_notice_period);

    // Creating instance of iam_team with shortcut 
    let iam_team: Team = create_team(String::from("IAM"), 9, true);


    // we can use shortcut name for variables too
    //  example - 

    let name = String::from("IAM");
    let number_of_members = 9;
    let is_technical = true;
    
    // Since the variables also use same names as fields
    // we can use same names like below
    let iam_team = Team {
        name,
        number_of_members,
        is_technical,
    };

// struct update syntax
// If we want to create a struct instance with same fields (copy the field from one to other instance)
// we shall use ..variable  (struct update syntax)

let ferrari = Car {
    name: String::from("ferrari"),
    color: String::from("Red"),
    price: 8.34,
};

// Now I want the price field to be copied from ferrari as its stack data
let lambo: Car = Car {
    name: String::from("lambohini"),
    color: String::from("Red"),
    ..ferrari
}; // In the above example we are using same price from ferrari but instead of manually creating lets add the same from ferrari

// This ..ferrari (struct update syntax works only with stack datatype. But to do with heap string datatype we should use .clone())
// This is because heap - String doesn't have copy trait it has only move trait so the onwership will be affected for name and color.
// however if i still want the same color (string)

let lambo: Car = Car {
    name: String::from("lambohini"),
    color: ferrari.color.clone(),
    ..ferrari
}; // This feature is helpful to copy multiple fields and since String has only move method to avoid messing with ownership we shall use .clone()


// 1. passing struct as parameter to function
// Passing struct instance as an argument to function would get the properties of them 
// but the ownership changes to function scope from instance

let baleno = Car {
    name: String::from("Baleno"),
    color: String::from("Imperial blue"),
    price: 8.99,
};

fn properties_of_car(car: Car){
    println!("1. NAME - {}\nCOLOR - {}\nPRICE - {}", car.name, car.color, car.price);
}

properties_of_car(baleno); // But this would change the ownership from baleno to properties_of_car and ends once the function ends
// println!("{}", baleno.color); // this would throw error as the ownership is moved

// 2. Mutable parameter
let baleno = Car {
    name: String::from("Baleno"),
    color: String::from("Imperial blue"),
    price: 8.99,
};

fn properties_of_car_mutable(mut car: Car) {
    println!("2. Price of car {} is {}", car.name, car.price);
    car.price = 9.98;
    println!("2. Increased price {}", car.price);
}


// In the above example we are mutating the car parameter so we just transfer the ownership and make it mutable
println!("2. {}", baleno.price);

properties_of_car_mutable(baleno); // the values would be changed along with ownership and soon the values goes out of scop
// println!("{}", baleno.price); // Though the value has been changed the variable out of scope as the ownership changed to function


// 3. Reference the parameter
// In this example we are passing the parameter as reference
// Thus the ownership is borrowed but the real ownership stayed with the instance baleno
let baleno = Car {
    name: String::from("Baleno"),
    color: String::from("Imperial blue"),
    price: 8.99,
};

fn properties_of_car_reference(car: &Car) {
    println!("3. Price of car {} is {}", car.name, car.price);
}

properties_of_car_reference(&baleno); // This just response the reference 
println!("3. Price of car {} is {}", baleno.name, baleno.price); // So the original ownership hasn't gone

// 4. Mutable reference
let mut baleno = Car {
    name: String::from("Baleno"),
    color: String::from("Imperial blue"),
    price: 8.99,
};

fn properties_of_car_mut_ref(car: &mut Car) {
    println!("4. Price of car {} is {}", car.name, car.price);
    car.price = 9.98;
    println!("4. Increased price {}", car.price);
}

properties_of_car_mut_ref(&mut baleno);
println!("4. {} is still valid with {{baleno.price}}", baleno.price);


// Once we derived debug trait above the struct we shall use debug interpolation in rust
let venue: Car = Car {
    name: String::from("Venue"),
    color: String::from("Imperial Blue"),
    price: 12.3,
};

println!("{:?}", venue);
println!("{:#?}", venue);

}