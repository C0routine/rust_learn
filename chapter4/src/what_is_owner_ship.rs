pub fn owner_ship_main() {
    {
        // Scrop 내에서 Ownership 는 유효
        // let owner = "Owner";
        // Scrop 밖에서는 OwnerShip 이 끝남
    }

    println!("@@@@ OwnerShip @@@@");
    owner_ship_string();
    println!("\n@@@@ OwnerShip Moving @@@@");
    owner_ship_move();
    println!("\n@@@@ OwnerShip Clone (with Heap) @@@@");
    owner_ship_clone_heap();
    println!("\n@@@@ Not OwnerShip Type (with Stack) @@@@");
    stack_type();
    println!("\n@@@@ OwnerShip Function @@@@");
    owner_ship_function();
}

// 소유권
fn owner_ship_string() {
    // 문자열 리터럴은 불변성이기에 Stack 에 할당
    // String 타입은 Heap 에 할당되어 있어서 가변성을 가짐, 그렇기에 메모리 할당 해제가 필요
    let mut owner_string = String::from("Owner");
    owner_string.push_str(" Ship");
    println!("{}", owner_string);

    // 소속된 스코프가 끝나면 drop 이 호출되어 메모리 해제
}

// 소유권 이동
fn owner_ship_move() {
    let owner_string = String::from("Heap Value");
    // owner_string 의 스택을 복사하여 new_owner_string 스택에 할당.
    // owner_string 변수를 무효화
    let new_owner_string = owner_string;

    // 메모리 중복 해제를 방지하기 위해 owner_string 은 유효하지 않음
    // println!("{}", owner_string);

    // 즉 new_owner_string 으로 owner_string 의 소유권이 이전됨
    println!("{}", new_owner_string);
}

// 소유권 Clone (Heap 영역 복사)
fn owner_ship_clone_heap() {
    let mut owner_string = String::from("Heap Clone Value");
    // clone 을 통해 owner_string Heap 의 값을 복사하여 새로운 Heap 에 할당
    // 즉 값은 복사되지만 서로 다른 소유권을 가짐
    let owner_string_copy: String = owner_string.clone();

    println!("origin: {}\ncopy: {}\n", owner_string, owner_string_copy);
    owner_string.push_str(" Origin Variable");

    // owner_string 의 값만 변경되었기에 owner_string_copy 의 값은 변경되지 않음
    println!("origin: {}\ncopy: {}\n", owner_string, owner_string_copy);
}

// 스택에 할당되는 타입 (소유권 개념 없음)
// 자유로운 복사 가능, Copy 트레이트를 가지면 소유권 개념이 없음
fn stack_type() {
    let stack_int: i64 = 10;
    let stack_float: f64 = 10.0;
    let stack_bool: bool = true;
    let stack_char: char = 'A';
    let stack_tuple: (i32, f64, bool) = (10, 10.0, true);

    let new_stack_int = stack_int;
    let new_stack_float = stack_float;
    let new_stack_bool = stack_bool;
    let new_stack_char = stack_char;
    let new_stack_tuple = stack_tuple;

    println!("origin: {}\tcopy: {}", stack_int, new_stack_int);
    println!("origin: {}\tcopy: {}", stack_float, new_stack_float);
    println!("origin: {}\tcopy: {}", stack_bool, new_stack_bool);
    println!("origin: {}\tcopy: {}", stack_char, new_stack_char);
    println!("origin: {:?}\tcopy: {:?}", stack_tuple, new_stack_tuple);
}

// 소유권 함수
fn owner_ship_function() {
    let origin_owner_string = String::from("OwnerShip");
    // 소유권이 owner_ship_function_move 로 이동
    owner_ship_function_move(origin_owner_string);
    // origin_owner_string 은 더 이상 유효하지 않음

    // 소유권이 owenr_string_return 으로 이동
    let owenr_string_return: String = {
        let owner_string = String::from("OwnerShip String");
        owner_string
    };

    // 소유권이 owner_ship_function_return 로 이동
    let (owner_string, size): (String, usize) = owner_ship_function_return(owenr_string_return);

    // owenr_string_return 은 더 이상 유효하지 않음
    println!("{}\t size: {}", owner_string, size);
}

// 소유권 함수 이동
fn owner_ship_function_move(move_owner_string: String) {
    println!("{} Function Move", move_owner_string);
}

// 소유권 함수 반환
fn owner_ship_function_return(mut owner_string: String) -> (String, usize) {
    owner_string.push_str(" Return Function");
    let length = owner_string.len();
    (owner_string, length)
}
