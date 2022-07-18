use std::io;
use super::*;

pub fn load_days() {
    println!("Enter the number of the day (1-25)");
    let mut day = String::new();
    io::stdin().read_line(&mut day)
        .expect("Could not read the input");
    
    match day.trim() {
        "1" => day_1::calc_measurements(),
        _ => println!("Could not match a day"),
    }
}
