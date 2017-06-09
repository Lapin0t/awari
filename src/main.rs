#![feature(plugin)]
#![feature(conservative_impl_trait)]
#![cfg_attr(test, plugin(quickcheck_macros))]


#[cfg(test)]
extern crate quickcheck;

mod utils;
mod awari;
//mod retrograde;

#[cfg(test)]
mod tests;


fn main() {
}
