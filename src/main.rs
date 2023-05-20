use std::io::stdin;

fn main() {
    println!("Enter the temperature in Fahrenheit");
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Error entering value");
    let farh: f64 =  input.trim().parse().expect("You didn't enter a number, I'm sorry, you're going to have to!");
    let celcius = ((farh - 32.0) * 5.0 / 9.0).floor();

    println!("{} Fahrenheit approximated in Celcius is {} degrees!", farh, celcius)  

}
