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

}