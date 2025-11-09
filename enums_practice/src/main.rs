// Enum with match passing associate value
enum LaundryCloth {
    Cold,
    Hot { temperature: u32},
    Delicate(String),
}

fn wash_cloth(state: LaundryCloth) {
    match state {
        LaundryCloth::Cold => println!("Cold Wash"),
        LaundryCloth::Hot { temperature} => println!("cloth washed in {temperature} celcius"),
        LaundryCloth::Delicate((material)) => println!("{material} is washing"),
    }
    
}

// Defining methods on enum 
// we can do the same with method
impl LaundryCloth {
    fn wash_cloth_method(&self) {
        match self {
        LaundryCloth::Cold => println!("Cold Wash"),
        LaundryCloth::Hot { temperature} => println!("cloth washed in {temperature} celcius"),
        LaundryCloth::Delicate((material)) => println!("{material} is washing"),
        }
    }
}

// Enum matching multiple values
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check_status(&self) {
        match self {
            OnlineOrderStatus::Delivered => {
                println!("Shipping delivered");
            }
            OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed | OnlineOrderStatus:: Shipped => {
                println!("Order is getting ready to deliver");
            }
        }
    }
}


// enum matching exact values
enum Milk{
    LowFat(u32),
    WholeFat,
}

impl Milk {
    fn check_fat(&self) {
        match self {
            Milk::LowFat(2) => {
                println!("2% LowFat milk");
            }
            Milk::LowFat(percent) => {
                println!("You got lowfat milk");
            }
            Milk::WholeFat => {
                println!("You've got WholeFat milk");
            }
        }
    }
}

// Enum aligned with match
#[derive(Debug)]
enum OperatingSystem {
    Windows,
    Macos,
    Linux,
}

// fn to match 

fn years_since_release(os: &OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => 39,
        OperatingSystem::Macos => 23,
        OperatingSystem::Linux => 34,
    }
}

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
    let os = OperatingSystem::Linux;
    let my_pc = years_since_release(&os);
    println!("{:?} released since {} years", os, my_pc);

    // Associate value match in enum

    let cloth = LaundryCloth::Cold;
    wash_cloth(cloth);

    let cloth = LaundryCloth::Hot { temperature: 100 };
    wash_cloth(cloth);

    let cloth = LaundryCloth::Delicate(String::from("Satin"));
    wash_cloth(cloth);

    // Do same with method in enum
    let cloth = LaundryCloth::Cold;
    cloth.wash_cloth_method();

    let cloth = LaundryCloth::Hot { temperature: 90 };
    cloth.wash_cloth_method();

    let cloth = LaundryCloth::Delicate(String::from("Cotton"));
    cloth.wash_cloth_method();


    // Enum for multiple match

    let order = OnlineOrderStatus::Ordered;
    order.check_status();

    let order = OnlineOrderStatus::Shipped;
    order.check_status();

    let order = OnlineOrderStatus::Delivered;
    order.check_status();

    // Exact match for enum

    Milk::WholeFat.check_fat();
    Milk::LowFat(2).check_fat();
    Milk::LowFat(5).check_fat();
}