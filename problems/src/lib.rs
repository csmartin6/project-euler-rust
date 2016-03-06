#![feature(test)]
extern crate test;

#[macro_use] 
extern crate itertools;

pub mod problems;
mod utils;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod benches;

