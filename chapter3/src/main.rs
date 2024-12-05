mod function;

fn main() {
    variables();
    data_types();
    function::function_start();
}

// Variables
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

// Data Types
fn data_types() {
    println!("\n@@@@@ Data Types @@@@@");
    let parse_type: u32 = "2112".parse().expect("error parse");
    println!("parseType: {}", parse_type);

    let mut scalar_int: i8 = 127;
    scalar_int = scalar_int.overflowing_add(1).0;
    let scalar_double: f64 = 3.14;
    let scalar_bool: bool = true;
    let scalar_char: char = 'A';
    let is_string: &str = "Hello, World!";
    println!(
        "scalarType: {scalar_int}, {scalar_double}, {scalar_bool}, {scalar_char}, {is_string}"
    );

    let tuple_type: (bool, &str, u64) = (true, "Hello, World!", 100);
    let (a, b, c) = tuple_type;
    {
        println!("tupleType Destructuring: {}, {}, {}", a, b, c);
    }
    println!(
        "tupleType Index: {}, {}, {}",
        tuple_type.0, tuple_type.1, tuple_type.2
    );
    println!("tupleType: {:?}", tuple_type);

    let months: [i8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    println!("arrayType: {:?}", months);

    let zero_array: [i64; 5] = [10; 5];
    println!("zeroArrayType: {:?}", zero_array);

    println!("first Month: {}", months[0]);
}
