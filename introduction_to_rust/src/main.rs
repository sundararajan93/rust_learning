mod variables_mutability;
mod data_types;

const TAX_RATE: f32 = 18.5;
fn main() {
    println!("Hello, world!");

    variables_mutability::variables_mut();

    println!("{TAX_RATE}");

    data_types::data_types_fun();
}
