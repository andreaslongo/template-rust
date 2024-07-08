//! # A binary crate

// Lints:
#![warn(clippy::pedantic)]
#![warn(deprecated_in_future)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

use template_rust::AppError;

fn main() -> Result<(), AppError> {
    let result = template_rust::add(2, 2);
    println!("2 + 2 = {result}");
    Ok(())
}
