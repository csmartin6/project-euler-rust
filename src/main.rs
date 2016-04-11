#![feature(test)]
extern crate test;

#[macro_use] 
extern crate itertools;

extern crate problems;
extern crate utils;


use std::env;
use std::process;
use std::fmt::Display;

// use problems;

fn run_problem<P,A: Display> (problem: P,name: &str) where P: Fn() -> A{
	let ans = problem();
	println!("The answer to {} is {}", name, ans);

}

fn main(){


    let argv = env::args().collect::<Vec<_>>();

    if argv.len() < 2 {
        println!("Usage: {} <problems>", argv[0]);
        process::exit(1);
    }

    for arg in argv.iter(){
    	match &arg[..] {
    		"problem_001" => run_problem(problems::problem_001::problem_001, arg),
    		"problem_002" => run_problem(problems::problem_002::problem_002, arg),
    		"problem_003" => run_problem(problems::problem_003::problem_003, arg),
    		"problem_004" => run_problem(problems::problem_004::problem_004, arg),
    		"problem_005" => run_problem(problems::problem_005::problem_005, arg),
            "problem_006" => run_problem(problems::problem_006::problem_006, arg),
            "problem_007" => run_problem(problems::problem_007::problem_007, arg),
            "problem_008" => run_problem(problems::problem_008::problem_008, arg),
            "problem_009" => run_problem(problems::problem_009::problem_009, arg),
            "problem_010" => run_problem(problems::problem_010::problem_010, arg),
    		_ => println!("Haven't done {} yet",arg)

    	}

    }

}