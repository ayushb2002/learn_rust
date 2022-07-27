pub fn switch_case_statement() {
    let i = 2;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1, 2"),
        3..=4 => println!("3, 4"),
        _ => println!("default!"),
    }
}

pub fn while_loop() {
    let mut i = 0;
    while i < 4 {
        println!("{}", i);
        i += 1;
    }
}

pub fn for_loop() {
    for i in 0..6 {
        println!("{}", i);
    }
}

pub fn if_else_statement() {
    let n = 3;
    if n > 0 {
        println!("Greater than 0!");
    } else if n < 0 {
        println!("Less than 0!");
    } else {
        println!("Equal to 0!");
    }
}
