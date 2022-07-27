#[derive(Debug)]
enum MyError {
    Error1,
}

fn divide(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::Error1)
    } else {
        Ok(dividend / divisor)
    }
}

pub fn results() {
    let divide = divide(4, 2);
    // let res = divide(2, 3).expect("Crashed");
    // println!("{}", res);
    match divide {
        Ok(v) => println!("{}", v),
        Err(v) => println!("{:?}", v),
    }

    // if divide.is_ok() {
    //     println!("{}", divide.unwrap());
    // }
}
