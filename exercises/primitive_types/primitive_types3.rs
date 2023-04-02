// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

fn main() {
    let range = 0..101;
    // check the type of range and print it
    println!("range is of type {:?}", range);
    let a: String = range.map(|x| x.to_string()).collect::<Vec<String>>().join(",");

    if a.len() >= 100 {
        println!("Wow, that's a big array!: {}", a);
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
