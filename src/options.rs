pub fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

pub fn options() {
    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    println!("{:?} unwraps to {}", divide1, divide1.unwrap());

    // if we will unwrap divide2 it will throw an exception as it won't be able to unwrap 'None' object. Thus rust will 'panic!'
    // println!("{:?} unwraps to {}" divide2, divide2.unwrap());
}
