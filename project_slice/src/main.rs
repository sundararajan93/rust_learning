/*
Define a `cereals` array with 5 heap Strings --> Done
  - Cookie Crisp
  - Cinnamon Toast Crunch
  - Frosted Flakes
  - Cocoa Puffs
  - Captain Crunch
 
Declare a `first_two` variable that extracts a slice
of the first two cereals. Print the slice. --> Done
 
Declare a `mid_three` variable that extracts a slice
of the middle three cereals (Cinnamon Toast Crunch
up to and including Cocoa Puffs). Print the slice. --> Done
 
Declare a `last_three` variable that extracts a slice
of the last three cereals. Print the slice. --> Done
 
Using the `last_three` slice, target the last element
("Captain Crunch") and replace it with "Lucky Charms".
Print the complete `cereals` array. --> Done
 
Declare a `cookie_crisp` variable that references the
"Cookie Crisp" String. --> Done
 
Declare a `cookie` variable that extracts a slice of
the text "Cookie" from the String. Print the slice. --> Done
 
Declare a `cocoa_puffs` variable. Make it a reference
to the "Cocoa Puffs" String (in other words, a &String). --> Done
 
Declare a `puffs` variable that extracts a slice of
the text "Puffs" from the String. Print the slice.
*/

fn main() {
  let mut cereals: [String; 5] = [String::from("Cookie Crisp"), String::from("Cinnamon Toast Crunch"), String::from("Frosted Flakes"), String::from("Cocoa Puffs"), String::from("Captain Crunch")];

  let first_two = &cereals[0..2];
  println!("{:?}", first_two);

  let mid_three = &cereals[1..4];
  println!("{:?}", mid_three);

  let last_three = &mut cereals[2..];
  println!("{:?}", last_three);

  last_three[2] = String::from("Lucky Charms");
  println!("{:?}", cereals);


  let cookie_crisp = &cereals[0];
  println!("{cookie_crisp:?}");

  let cookie = &cookie_crisp[..6];
  println!("{cookie:?}");

  let cocoa_puffs = &cereals[3];
  println!("{cocoa_puffs:?}");

  let puffs = &cocoa_puffs[6..];
  println!("{puffs:?}");
  
        
}
