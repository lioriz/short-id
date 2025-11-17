use short_id::{id, short_id, ShortId};
#[cfg(feature = "std")]
use short_id::{ordered_id, short_id_ordered};

fn main() {
    println!("=== Functions ===");

    // Generate a random ID using the function
    let random_id = short_id();
    println!("short_id():         {random_id}");

    // Generate a time-ordered ID using the function
    #[cfg(feature = "std")]
    {
        let ordered = short_id_ordered();
        println!("short_id_ordered(): {ordered}");
    }

    println!("\n=== Macros ===");

    // Generate IDs using macros (convenient shorthand)
    let macro_id = id!();
    println!("id!():              {macro_id}");

    #[cfg(feature = "std")]
    {
        let macro_ordered = ordered_id!();
        println!("ordered_id!():      {macro_ordered}");
    }

    println!("\n=== Typed Wrapper ===");

    // Generate IDs using the ShortId type
    let typed_random = ShortId::random();
    println!("ShortId::random():  {typed_random}");

    #[cfg(feature = "std")]
    {
        let typed_ordered = ShortId::ordered();
        println!("ShortId::ordered(): {typed_ordered}");
    }

    // Demonstrate type conversions
    let s: String = typed_random.clone().into();
    println!("\nConverted to String: {s}");
    println!("Using as_str():      {}", typed_random.as_str());
    println!("Using AsRef<str>:    {}", typed_random.as_ref());
}
