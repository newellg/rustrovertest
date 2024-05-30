use std::{io::{self}};


// Translation of Band Name Generator from Python Course


fn main() {
    let mut city = String::new();
    let mut pet :String = String::new();
    println!("Welcome to the Band Name Generator");
    println!("What city did you grow up in? : ");
    io::stdin().read_line(&mut city).expect("Read Line Failed.");
    let city = city.trim();
    println!("What is the name of your pet? : ");

    io::stdin().read_line(&mut pet).expect("Read Line Failed.");
    println!("Your band name could be {} {}", city, pet);
}



