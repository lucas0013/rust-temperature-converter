use std::io;

enum Units {
    Celsius = 1,
    Fahrenheit = 2,
}

fn celsius_to_fahrenheit(celsius: i64) -> i64 {
    (celsius * 9 / 5) + 32
}

fn fahrenheit_to_celsius(fahrenheit: i64) -> i64 {
    (fahrenheit - 32) * 5 / 9
}

fn main() {
    println!("---------- ❄️  Temperature Converter v0.1 🔥 ----------");

    let input_unit: u8 = loop {
        let mut input_unit = String::new();
        println!("You want to convert the value from what unit? 😎");
        println!("\n1. Celsius ℃");
        println!("2. Fahrenheit ℉");

        io::stdin()
            .read_line(&mut input_unit)
            .expect("\nFailed to read line ❌");

        match input_unit.trim().parse::<u8>() {
            Ok(num) => {
                if num == 1 || num == 2 {
                    break num;
                } else {
                    println!("\nPlease provide a valid value (1 or 2). 😐");
                    continue;
                }
            }
            Err(_) => {
                println!("\nPlease enter a valid number! 🤨");
                continue;
            }
        };
    };

    let input_value: i64 = loop {
        let mut input_value = String::new();
        println!("\nEnter a value to convert. 😎");

        io::stdin()
            .read_line(&mut input_value)
            .expect("\nFailed to read line ❌");

        match input_value.trim().parse::<i64>() {
            Ok(num) => break num,
            Err(_) => {
                println!("\nPlease enter a valid number! 🤨");
                continue;
            }
        };
    };

    if input_unit == Units::Celsius as u8 {
        println!("\nThe value has been converted from Celsius to Fahrenheit!! 🤖");
        println!(
            "\nFrom: {} °C to {} °F\n",
            input_value,
            celsius_to_fahrenheit(input_value)
        )
    } else if input_unit == Units::Fahrenheit as u8 {
        println!("\nThe value has been converted from Fahrenheit to Celsius!! 🤖");
        println!(
            "\nFrom: {} °F to {} °C\n",
            input_value,
            fahrenheit_to_celsius(input_value)
        );
    }
}
