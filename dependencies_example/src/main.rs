// A dependency is an external library crate that we pull into our project. Our code depends on it to run.

mod inventory;

// using User struct from inventory module
use inventory::User;
// using Fake, Faker struct from fake (external module), This module is added in dependencies Cargo.toml file
use fake::{Fake, Faker};

fn main() {
    // Creating Dummy data with fake() method for User struct
    let dummy_user: User = Faker.fake();
    println!("{dummy_user:?}");
}