// Create a struct and derive Debug code so
// all fields of the struct can be printed.
#[derive(Debug)]
struct MyStruct {
    i: i32,
    s: String,
}

fn main() {
    // Create an instance of the struct
    let ms = MyStruct {
        i: 123,
        s: "hello".to_string(),
    };

    // Print the fields manually, works without the derive(Debug)
    println!("MyStruct: i={} s={}", ms.i, ms.s);

    // Print using the Debug derived code:
    println!("MyStruct={:?}", ms);
}

