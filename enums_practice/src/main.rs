// Enum nested inside enum
#[derive(Debug)]
enum Environments {
    Dev,
    Uat,
    Prod,
}

#[derive(Debug)]
enum Deploy {
    Connect(Environments),
    Error(String),
}

// Enum nested inside struct

#[derive(Debug)]
enum Status {
    OK,
    NotFound,
    TimedOut,
}

#[derive(Debug)]
struct Request {
    url: String,
    user: String,
    passwd: String,
    status: Status,
}

#[derive(Debug)]
enum InstanceStatus {
    Connected,
    Disconnected,
    Error(String),
}

#[derive(Debug)]
struct Instance {
    name: String,
    cpu: u32,
    hdd: u32,
    status: InstanceStatus,
}


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


    let my_web_server = Instance {
        name: String::from("Web Server"),
        cpu: 2,
        hdd: 512,
        status: InstanceStatus::Connected,
    };

    println!("{:#?}", my_web_server);


    // enum inside enum

    let deployment = Deploy::Connect(Environments::Dev);
    println!("{:#?}", deployment);

    // Enum inside struct

    let request = Request {
        url: String::from("https://example.com/login"),
        user: String::from("testuser"),
        passwd: String::from("testpassword@123"),
        status: Status::OK,
    };

    println!("{:?}", request);
}