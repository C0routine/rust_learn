use std::io;
mod celsius;
mod fahrenheit;

mod common {
    pub mod temperature_enum;
}

use common::temperature_enum::Temperature;

fn main() {
    println!("Convert Temperature Program");
    println!("@@@Support Only: Celsius <-> Fahrenheit\n");
    print!("1. Celsius to Fahrenheit\n2. Fahrenheit to Celsius\n0. Exit\n");

    loop {
        eprint!("\nSelect Menu Number: ");

        let mut select_number = String::new();
        io::stdin()
            .read_line(&mut select_number)
            .expect("Failed to read line");

        let select_menu: i8 = match select_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only Input Support Number!!");
                continue;
            }
        };

        match select_menu {
            0 => {
                println!("Good Bye~~!");
                break;
            }
            1 => fahrenheit::fahrenheit_convert(Temperature::Celsius),
            2 => celsius::celsisu_convert(Temperature::Fahrenheit),
            _ => {
                println!("Invalid Menu Number!!");
                continue;
            }
        }
    }
}
