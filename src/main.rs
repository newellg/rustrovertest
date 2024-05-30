use std::{io::{self}};


// Translation of Band Name Generator from Python

fn say_band_name(place: &str, name: &str) {
    println!("Your band name could be {} {}", place, name);
}
fn main() {
    let mut city = String::new();
    let mut pet :String = String::new();
    println!("Welcome to the Band Name Generator");
    println!("What city did you grow up in? : ");
    io::stdin().read_line(&mut city).expect("Read Line Failed.");
    let trimmed_city = city.trim();
    println!("What is the name of your pet? : ");

    io::stdin().read_line(&mut pet).expect("Read Line Failed.");
    say_band_name(&trimmed_city, &pet );
}


