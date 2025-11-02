// enum is finite set of possible values
#[derive(Debug)]
enum CardSuit {
    Heart,
    Diamond,
    Clover,
    Spade,
}

struct Card {
    ranking: String,
    suite: CardSuit, // We could use the enum as struct type for a field
}


// enum with associated value 
#[derive(Debug)]
enum PaymentmethodType {
    CreditCard(String), // Enum can have one or more associated value defined like this.
    DebitCard,
    Paypal(String, String), // Suppose we need more than one associated value an enum Variant can support that too
}

fn main() {
    
    let first_card = CardSuit::Heart;
    println!("{:?}", first_card);

    let mut second_card: CardSuit = CardSuit::Clover;
    println!("{:?}", second_card);
    

    second_card = CardSuit::Spade;
    println!("{:?}", second_card);

    // second_card = "Test"; // This wouldn't work as they already defined with CardSuit type

    // We could use this enum as a function paramter, variable, ownership is similar, we can use enum in array tuple, strcut

    let card_suits = [CardSuit::Clover, CardSuit::Diamond];

    let card_suit_tuple = (CardSuit::Spade, 6, String::from("King"));

    // When we want associated (Additional value associated to enum) we shall use below tuple syntax
    // let visa = (PaymentmethodType::CreditCard, String::from("4587-5648"));

    // But enums have associated value can be declared in enum itself

    let visa = PaymentmethodType::CreditCard(String::from("4578-9856"));

    println!("{:?}", visa);

    let payment_type = PaymentmethodType::Paypal(String::from("sundar@rustmail.com"), String::from("Mydummypasswd"));
    println!("{:?}", payment_type);

}