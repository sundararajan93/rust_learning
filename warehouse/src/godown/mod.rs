pub mod parking;

// module godown is created as folder and within that mod.rs file should be added with module code
pub const MANAGER: &str = "Harris";


// We can add pub to all the constructs like enum struct etc
#[derive(Debug)]
pub enum Category {
    Ladder,
    Hammer,
}

#[derive(Debug)]
pub struct Inventory {
    name: String,
    quantity: u32,
    category: Category,
}

// impl keyword doesn't need to be public function within them could be
impl Inventory {
    pub fn new(name: String, quantity: u32, category: Category) -> Self {
        Self {
            name,
            quantity,
            category,
        }
    }
}

