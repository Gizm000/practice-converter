use std::{io, io::Write, thread::sleep, time};

fn text(message: &str, delay: u64) {

    for letter in message.chars() {
        //* flush stdout to allow single character prints
        io::stdout().flush().unwrap();
        print!("{}", letter);
        sleep(time::Duration::from_millis(delay));
    }
    println!()
}
fn main() {
    text( "Welcome to the convertorium", 35 ); 
    text( "Please select one of the options below", 25 );
    menu();
    loop {
        let mut options = String::new();
        
        io::stdin()
            .read_line(&mut options)
            .expect("Ah ah ah, you didn't say the magic word");
            
        let options: u32 = match options.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error! Please enter one of the options"); continue;
            }
        };
        match options {
            1 => f_to_c(),
            2 => c_to_f(),
            3 => exit(),
            _ => println!("Error! Please enter one of the options"),
        }
    }
}
fn f_to_c() {
    println!("Please enter Fahrenheit value that you'd like to convert");
    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature)
        .expect("Ah ah ah, you didn't say the magic word");
    let temperature: f64 = match temperature.trim().parse() {
        Ok(f) => f,
        Err(_) => {println!("Error! Please enter one of the options"); return;}
    };

    let converted = (temperature - 32.0) / 1.8;
    println!("{:.2} Celsius", converted);
}
fn c_to_f() {
    println!("Please enter Celcius value that you'd like to convert");
    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature)
        .expect("Ah ah ah, you didn't say the magic word");
    let temperature: f64 = match temperature.trim().parse() {
        Ok(f) => f,
        Err(_) => {println!("Error! Please enter one of the options"); return;}
    };
    let converted = temperature * 1.8 + 32.0; //*I want to replace the 'temperature' with function */
    println!("{:.2} Fahrenheit", converted);
}
fn exit() {
    std::process::exit(0);
}
fn menu() {
    println!("1: Convert value from Fahrenheit to Celcius");
    println!("2: Convert value from Celcius to Fahrenheit");
    println!("3: Exit the converter");
}
//TODO: Need to figure out how to get below to work within function and then input it into other fn's
// fn input(temperature: i32) -> i32 {
//     let mut temperature = String::new();
//     io::stdin().read_line(&mut temperature)
//         .expect("Ah ah ah, you didn't say the magic word");
//     let temperature: f64 = match temperature.trim().parse() {
//         Ok(f) => f,
//         Err(_) => {println!("Error! Please enter one of the options"); return;}
//     }
// }