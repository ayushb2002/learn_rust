pub fn print_variables() {
    let emoji = "\u{1F600}";
    println!("Hello, world! {}", emoji);
}

pub fn call_is_even() {
    let num: u8 = 7;
    println!("Is {} even? \nAnswer - {}", num, is_even(num));
}

pub fn is_even(num: u8) -> bool {
    let res: u8 = num % 2;
    res == 0
}
