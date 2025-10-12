/*
Define a `color_to_number` function that accepts a 'color'
parameter (a string). Use if, else if, and else
statements to return a corresponding numeric value based
on the following rules:
1. If the color is "red", return 1.
2. If the color is "green", return 2.
3. If the color is "blue", return 3.
4. If the color is any other string, return 0.  ==> Done
 
Refactor the function above to use the `match` statement
instead of if, else if, and else. ==> Done
 
Define a `factorial` function that calculates the
factorial of a number. The factorial is the product
of multiplying a number by every incremental
number leading up to it, starting from 1.
 
Examples:
The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
factorial(5) should return 120.
 
The factorial of 4 is 4 * 3 * 2 * 1 = 24
factorial(4) should return 24.
 
 var = 4 * (4 - 1)
 var = var * (3 - 1)
 var = var * ( 2 - 1)
 

Implement two solutions/functions for the problem.
The first solution should not use recursion.
The second solution should use recursion.
*/

use core::num;
use std::iter::Product;

fn factorial(number: i32) -> i32 {
    let mut count = number; // 5
    let mut product = 1; // 1
    while count > 0 {
        product *= count; // 5 * 1 = 5 -> 5 * 4 = 20 -> 20 * 3 = 60 -> 60 * 2 = 120 -> 120 * 1 = 120
        count -= 1; // 5 - 1 = 4 -> 4 - 1 = 3 -> 3 - 1 = 2 -> 2 - 1 = 1 -> 1 - 1 = 0
    }
    return product;
}

fn factorial_recursion(number: i32) -> i32{
    if number == 1 {
        return 1;
    }
       number * factorial_recursion(number - 1) // 5 * factorial(4)
}

fn color_to_number(color: &str) -> i32 {
        // if color == "red" {
        //     1
        // } else if color == "green" {
        //     2
        // } else if  color == "blue" {
        //     3
        // } else {
        //     0
        // }
        match color {
        "red" => 1,
        "green" => 2, 
        "blue" => 3,
        _ => 0,
    }
    }


fn main() {
    println!("{}", color_to_number("purple"));

    println!("{}", factorial(5));
    println!("{}", factorial(4));

    println!("{}", factorial_recursion(5));
    println!("{}", factorial_recursion(4));

}
