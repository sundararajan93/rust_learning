use fake::Dummy;

#[derive(Debug, Dummy)]
pub struct User {
    name: String,
    user_id: String,
    email_id: String,
    emp_no: u32,
}
