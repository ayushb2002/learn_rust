#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C { x: i32, y: i32 },
}

pub fn call_enum() {
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5);
    let c: MyEnum = MyEnum::C { x: 10, y: 20 };
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}

pub fn compare_enum_values() {
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5);
    let c: MyEnum = MyEnum::C { x: 10, y: 20 };

    if let MyEnum::B(val) = b {
        println!("{}", val);
    }

    if let MyEnum::C { x, y } = c {
        println!("{} {}", x, y);
    }
}
