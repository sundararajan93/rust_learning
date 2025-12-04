// Using the external module fake and getting the Dummy Trait 
// use Dummy Trait for the struct 
use fake::Dummy;

// Adding Dummy trait so that we shall create a dummy value for the struct
#[derive(Debug, Dummy)]
pub struct User {
    name: String,
    user_id: String,
    email_id: String,
    emp_no: u32,
}
