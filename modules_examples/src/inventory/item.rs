pub const MANAGER: &str = "LOIS";

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub count: u32,
}

impl Item {
    pub fn new(name: String, count: u32) -> Self {
        super::greeting_owner();
        println!("Created Item");
        Self {
            name,
            count,
        }
    }
}