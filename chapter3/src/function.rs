// Function
pub fn function_start() {
    println!("\n@@@@@ Function @@@@@");
    function_naming();
    function_parameters(10, 30);
    function_expression();
    println!(
        "function_expression_return: {}",
        function_expression_return()
    );
    println!(
        "function_expression_return2: {}",
        function_expression_return2()
    );
}

fn function_naming() {
    println!("function_naming");
}

fn function_parameters(x: i32, y: i32) {
    println!("function_parameters {}", x + y);
}

fn function_expression() {
    // 구문
    // let statements = 20;

    // 표현식
    let expression = {
        let only_scope = 10;
        only_scope + 20
    };
    println!("function_expression: {}", expression);
}

fn function_expression_return() -> i32 {
    // return 키워드를 사용하지 않아도 마지막 표현식이 반환.
    77
}

fn function_expression_return2() -> i32 {
    let x = 10;
    // ; 세미콜론을 붙이면 구문으로 인식되어 반환되지 않음.
    // x + 777;
    // return 키워드를 사용하면 반환됨.
    return x;
}
