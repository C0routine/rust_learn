fn main() {
    variables();
    data_types();
}

fn variables() {
    println!("\n@@@@@ Variables @@@@@");

    // Mutable Variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 10;
    println!("The value of x is: {}", x);

    // Constant Variables
    const ONE_DAY_SECONDS: u32 = 60 * 60 * 24;
    println!("One day has {} seconds", ONE_DAY_SECONDS);

    // Shadowing Variables
    let origin = 10;
    let origin = origin + 20;
    {
        // Scope 내부 변수
        let origin = origin * 2;
        println!("The value of origin in the inner scope is: {origin}"); // 60
    }
    println!("The value of origin is: {origin}"); // 30

    // Shadowing with different types
    let target: &str = "Hello, World!";
    println!("target: {}", target);
    let target: usize = target.len();
    println!("target: {}", target);
}

fn data_types() {
    println!("\n@@@@@ Data Types @@@@@");
    let parse_type: u32 = "2112".parse().expect("error parse");
    println!("parseType: {}", parse_type);
}
