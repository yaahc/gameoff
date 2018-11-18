#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::type_complexity, clippy::new_ret_no_self)
)]
extern crate amethyst;
extern crate rand;

mod component;
mod load;
mod map;
pub mod state;
pub mod system;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
