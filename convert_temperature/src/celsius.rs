use crate::common::temperature_enum::Temperature;
use std::io;

// 온도 타입에 맞는 변환 함수 호출
pub fn celsisu_convert(temperature: Temperature) {
    match temperature {
        Temperature::Fahrenheit => fahtenheit_to_celsius(),
        _ => {
            println!("Invalid Temperature Type!!");
        }
    }
}

// 화씨를 섭씨로 변환
fn fahtenheit_to_celsius() {
    loop {
        eprint!("Input Fahrenheit: ");
        let mut input_fahrenheit = String::new();
        io::stdin()
            .read_line(&mut input_fahrenheit)
            .expect("Failed to read line");

        let input_fahrenheit: f64 = match input_fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only Input Support Number!!");
                continue;
            }
        };

        let celsius = (input_fahrenheit - 32.0) * 5.0 / 9.0;
        println!(
            "Fahrenheit: {}°F -> Celsius: {}°C",
            input_fahrenheit, celsius
        );
        return;
    }
}
