use short_id::{id, ordered_id};

fn main() {
    // Use the convenient macro syntax
    let random = id!();
    println!("Random ID (macro):  {random}");

    let ordered = ordered_id!();
    println!("Ordered ID (macro): {ordered}");

    // They work exactly the same as the functions
    assert_eq!(random.len(), 14);
    assert_eq!(ordered.len(), 14);
}
