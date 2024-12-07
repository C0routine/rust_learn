use rand::{rng, Rng};

// control flow (if-else)
pub fn control_start() {
    println!("\n@@@@@ Control Flow @@@@@");

    let ready_status: i32 = rng().random_range(-100..=100);
    if ready_status > 0 {
        println!("i'm ready");
    } else {
        println!("i'm not ready");
    }

    let number_flow: i32 = rng().random_range(-100..100);
    if number_flow < 0 {
        println!("Number Flow minus");
    } else if number_flow == 1 {
        println!("Number == 1");
    } else {
        println!("Number Flow plus");
    }

    let is_max = if number_flow == 100 { true } else { false };
    println!("Number isMax? {}", is_max);

    control_loop();
    control_while();
    control_for();
}

// loop
fn control_loop() {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };
    println!("result : {}", result);

    let mut count = 0;
    'count_first: loop {
        println!("count_first : {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'count_first;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("count : {count}");
}

// while
fn control_while() {
    let mut count = 5;

    while count > 0 {
        print!("{count}..");
        count -= 2;
    }
    println!("while out!");
}

// for
fn control_for() {
    let random_array: [i32; 10] = [rng().random_range(1..100); 10];

    for rand in random_array {
        print!("{rand}..");
    }
    print!("\n");

    for number in (1..10).rev() {
        let newline = if number == 1 { "END\n" } else { "" };
        print!("{number}..{}", newline);
    }
}
