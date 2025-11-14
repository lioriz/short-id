use short_id::short_id;
#[cfg(feature = "std")]
use short_id::short_id_ordered;

fn main() {
    // Generate a random ID (works in both std and no_std)
    let random_id = short_id();
    println!("Random ID: {random_id}");

    // Generate a time-ordered ID (requires std feature)
    #[cfg(feature = "std")]
    {
        let ordered_id = short_id_ordered();
        println!("Time-ordered ID: {ordered_id}");
    }

    #[cfg(not(feature = "std"))]
    {
        println!("Time-ordered ID: (not available in no_std mode)");
    }
}
