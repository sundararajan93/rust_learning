pub mod item;
pub const STORE_OWNER: &str = "PAUL";
pub const MANAGER: &str = "JAMES";

fn greeting_owner() {
    println!("Hello, {}", STORE_OWNER);
}