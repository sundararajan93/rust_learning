/*
Define a Flight struct with the following fields: --> Done
  - an `origin` field (String)
  - a `destination` field (String)
  - a `price` field (f64)
  - a `passengers` field (u32)
 
Derive a Debug trait implementation for the Flight struct. --> Done
 
Define a `new` constructor function that returns a new
instance of a Flight. --> Done
 
Define a `change_destination` method that accepts a new
destination and overwrites the value of the `destination`
field. --> Done
 
Define a `increase_price` method that raises the value
of the `price` by 20% (multiply the `price` field by 1.20).
Make sure to save the new `price` field value. --> Done
 
Define a `itinerary` method that prints out both the
`origin` and `destination` fields in the following format
(origin -> destination). --> Done
 
Use the constructor function to create a new Flight instance
in the main function. Invoke all of the defined methods.
Print out the struct in Debug format to confirm the struct
updates as you expect. --> Done
 
Use struct update syntax to copy the `price` and `passengers`
fields to a new Flight struct instance. Make sure to provide
new Strings for the remaining fields to ensure ownership
doesn't transfer. Assign the new Flight to a separate variable. --> Done
*/

#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(self: &mut Self, new_destination: String) {
        println!("{:#?}", self);
        self.destination = new_destination;
        println!("{:#?}", self);
    }

    fn increase_price(self: &mut Self) {
        self.price *= 1.20;
        println!("{:#?}", self);

    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn main() {
    let mut flight_a: Flight = Flight::new(String::from("Chennai"), String::from("Delhi"), 45.6, 60);

    flight_a.change_destination(String::from("Bangalore"));

    flight_a.increase_price();
    flight_a.itinerary();

    let mut flight_b = Flight {
        origin: String::from("Kochi"),
        destination: String::from("Delhi"),
        .. flight_a
    };

    println!("{flight_b:#?}");


}
