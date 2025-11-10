/*
Define a Tier enum with three variants: Gold, Silver,
Platinum. Derive a Debug implementation for the Tier enum. --> Done
 
Define a Subscription enum with three variants: Free,
Basic, and Premium. Derive a Debug implementation for the
Subscription enum. --> Done
 
The Free variant should have no associated data.
 
The Basic variant should be a tuple variant with two pieces
of data. The first one should be a f64 (the price per month)
and the second one should be a u32 (the number of months). --> Done
 
The Premium variant should be a struct variant with a 'tier'
field. The tier field should be a Tier enum value. --> Done
 
Define a 'summarize' method on the Subscription enum. --> Done
 
If the Subscription is Free, output the text "You have
limited access to the site". --> Done
 
If the Subscription is Basic, output the text "You have
limited access to the site's premium features for {price}
for {months} months", where {price} amd {months} come from
the tuple variant's associated data. --> Done
 
If the Subscription is Premium, output the text "You have
full access to the site's premium features. Your tier is
{tier:?}"", where {tier} comes from the struct variant's
associated 'tier' field. --> Done
 
In the `main` function, create 3 instances of Subscription,
each one with a different variant. Invoke the `summarize`
method on each instance. --> 
*/

#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium {tier: Tier},
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::Free => println!("You have limited access to the site"),
            Subscription::Basic(price_per_month, number_of_months) => println!("You have limited access to the site's premium features for {price_per_month} for {number_of_months} months"),
            Subscription::Premium{tier} => println!("You have full access to the site's premium features. Your tier is {tier:?}"),
        }
    }
}

fn main() {
    
    let free_subscription1: Subscription = Subscription::Free;
    free_subscription1.summarize();

    let basic_subscription: Subscription = Subscription::Basic(499.40, 12);
    basic_subscription.summarize();

    let premium_subscription: Subscription = Subscription::Premium {
        tier: Tier::Platinum
    };
    premium_subscription.summarize();
    
}
