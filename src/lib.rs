//! A library crate

#![warn(
    clippy::pedantic,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms
)]

/// An example function which adds left and right.
/// ```
/// # use template_rust::add;
/// add(1, 2) == 3;
/// add(5, 5) == 10;
/// ```
#[must_use]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
