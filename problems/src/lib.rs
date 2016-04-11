#![feature(step_by)]


#![feature(test)]
extern crate test;

#[macro_use] 
extern crate itertools;
extern crate utils;

pub mod problems;



#[cfg(test)]
mod tests;

#[cfg(test)]
mod benches;


pub mod problem_009;
pub mod problem_010;

