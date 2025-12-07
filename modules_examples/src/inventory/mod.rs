//! This module is about the items
pub mod item;
/// This constant is store owner
pub const STORE_OWNER: &str = "PAUL";
/// This constant is store owner
pub const MANAGER: &str = "JAMES";

/// This function is to greet the owner
fn greeting_owner() {
    println!("Hello, {}", STORE_OWNER);
}