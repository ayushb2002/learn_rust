use std::collections::HashMap;

pub fn vectors() {
    let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5];
    let mut n = vec.len();
    println!("Length of vector before push: {}", n);
    vec.push(6);
    n = vec.len();
    println!("Length of vector after push: {}", n);
    vec.remove(0);
    println!("Length of vector after remove: {}", n);
    println!("Vector after remove: {:?}", vec);
}

pub fn maps() {
    let mut map = HashMap::new();
    map.insert(0, "H1");
    map.insert(1, "H2");

    println!("{:?}", map);

    match map.get(&0) {
        Some(str) => println!("{}", str),
        _ => println!("Doesn't exist in map!"),
    }

    match map.get(&2) {
        Some(str) => println!("{}", str),
        _ => println!("Doesn't exist in map!"),
    }

    map.remove(&0);
    println!("{:?}", map);
}
