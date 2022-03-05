use std::io;

// This program is to practice my first days in RUST
fn main() {

    let mut user_input = String::new();


    println!("Convert degree celcius to Faraheint!");
    io::stdin()
        .read_line(&mut user_input)
        .unwrap();
        // Here I am trying to convert the input into a  float
        let celcius : f32 = user_input.trim().parse().unwrap();

        // Convert the user input(Celcius) to Fahrenheit
        let faraheint = (9 as f32/ 5 as f32)*celcius + 32 as f32;

    println!("{} Celcius is equal to {} Faraheint",celcius,faraheint)
}