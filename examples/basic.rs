use short_id::short_id;
use short_id::short_id_ordered;

fn main() {
    // Generate a random ID
    let random_id = short_id();
    println!("Random ID: {}", random_id);

    // Generate a time-ordered ID
    let ordered_id = short_id_ordered();
    println!("Time-ordered ID: {}", ordered_id);
}
