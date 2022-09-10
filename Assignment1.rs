/****************************************************************************************************
Programmer: Josh Brown | 822455771
CS 420

9/10/2022 ~~ DUE DATE
~~ Assignment 1 : Write a Rust function increase that accepts a 64-bit int, adds one to the value,
and returns the result.
******************************************************************************************************/

fn main() {

    fn increase(mut x :i64) -> i64 {
        x = x + 1;
        return x
    }

    let y = 6;
    println!("y is: {}", y);
    let printval = increase(y);

    println!("y after increase: {}", printval);

}