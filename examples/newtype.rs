use short_id::ShortId;

fn main() {
    // Create a random ID
    let id1 = ShortId::random();
    println!("Random ID: {}", id1);

    // Create a time-ordered ID
    let id2 = ShortId::ordered();
    println!("Ordered ID: {}", id2);

    // Access as string slice
    let s: &str = id1.as_str();
    println!("As str: {}", s);

    // Use AsRef<str>
    print_id(&id2);

    // Convert to String
    let string: String = id1.clone().into_string();
    println!("Into String: {}", string);

    // Create from String
    let id3: ShortId = string.into();
    println!("From String: {}", id3);

    // Compare IDs (PartialEq, Ord)
    let id4 = ShortId::random();
    let id5 = ShortId::random();
    println!("\nIDs are equal: {}", id4 == id5);
    println!("ID4 < ID5: {}", id4 < id5);
}

fn print_id<T: AsRef<str>>(id: &T) {
    println!("Using AsRef: {}", id.as_ref());
}
