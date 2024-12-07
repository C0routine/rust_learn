use crate::common::temperature_enum::Temperature;
use std::io;

// 온도 타입에 맞는 변환 함수 호출
pub fn fahrenheit_convert(temperature: Temperature) {
    match temperature {
        Temperature::Celsius => celsisu_to_fahrenheit(),
        _ => {
            println!("Invalid Temperature Type!!");
        }
    }
}

// 섭씨를 화씨로 변환
fn celsisu_to_fahrenheit() {
    loop {
        eprint!("Input Celsius: ");
        let mut input_celsius = String::new();
        io::stdin()
            .read_line(&mut input_celsius)
            .expect("Failed to read line");

        let input_celsius: f64 = match input_celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only Input Support Number!!");
                continue;
            }
        };

        let fahrenheit = (input_celsius * 9.0 / 5.0) + 32.0;
        println!(
            "Celsius: {}°C -> Fahrenheit: {}°F",
            input_celsius, fahrenheit
        );
        return;
    }
}
