pub fn strings_and_slices() {
    //let str: &str = "hello world";
    let mut string: String = String::from("Hello World"); // dynamic sized array object for string!

    //let slice = &string[..6];
    string.push_str("! This is a trial command!");
    string = string.replace("Hello", "Bye");
    println!("{}", string);
}

pub fn arrays_and_slices() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    println!("Length of array: {}", arr.len());
    println!("Array: {:?}", arr);

    let slice = &arr[1..3];
    println!("Slice of array: {:?}", slice);
}
