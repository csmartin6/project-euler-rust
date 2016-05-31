#![feature(alloc_system)]
#![feature(test)]

extern crate alloc_system;
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


    let mut argv = env::args().collect::<Vec<_>>();

    if argv.len() < 2 {
        println!("Usage: {} <problems>", argv[0]);
        process::exit(1);
    }
    let problems = argv.split_off(1); 
    for arg in problems.iter(){
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
            "problem_011" => run_problem(problems::problem_011::problem_011, arg),            
            "problem_012" => run_problem(problems::problem_012::problem_012, arg),            
            "problem_013" => run_problem(problems::problem_013::problem_013, arg),            
            "problem_014" => run_problem(problems::problem_014::problem_014, arg),            
            "problem_015" => run_problem(problems::problem_015::problem_015, arg),            
            "problem_016" => run_problem(problems::problem_016::problem_016, arg),
            "problem_017" => run_problem(problems::problem_017::problem_017, arg),            
            "problem_018" => run_problem(problems::problem_018::problem_018, arg),            
            "problem_019" => run_problem(problems::problem_019::problem_019, arg), 
            "problem_020" => run_problem(problems::problem_020::problem_020, arg),
            "problem_021" => run_problem(problems::problem_021::problem_021, arg),            
            "problem_022" => run_problem(problems::problem_022::problem_022, arg),            
            "problem_023" => run_problem(problems::problem_023::problem_023, arg),            
            "problem_024" => run_problem(problems::problem_024::problem_024, arg),            
            "problem_025" => run_problem(problems::problem_025::problem_025, arg),
            "problem_026" => run_problem(problems::problem_026::problem_026, arg),  
            "problem_027" => run_problem(problems::problem_027::problem_027, arg),  
            "problem_028" => run_problem(problems::problem_028::problem_028, arg),  
            "problem_029" => run_problem(problems::problem_029::problem_029, arg),  
            "problem_030" => run_problem(problems::problem_030::problem_030, arg),  
            "problem_031" => run_problem(problems::problem_031::problem_031, arg),  
            "problem_032" => run_problem(problems::problem_032::problem_032, arg),
            "problem_033" => run_problem(problems::problem_033::problem_033, arg),  
            "problem_034" => run_problem(problems::problem_034::problem_034, arg),  
            "problem_035" => run_problem(problems::problem_035::problem_035, arg),  
            "problem_036" => run_problem(problems::problem_036::problem_036, arg),
            "problem_037" => run_problem(problems::problem_037::problem_037, arg),  
            "problem_038" => run_problem(problems::problem_038::problem_038, arg),
            "problem_039" => run_problem(problems::problem_039::problem_039, arg),   
            "problem_040" => run_problem(problems::problem_040::problem_040, arg),
            "problem_041" => run_problem(problems::problem_041::problem_041, arg),   
            "problem_042" => run_problem(problems::problem_042::problem_042, arg),
            "problem_043" => run_problem(problems::problem_043::problem_043, arg),   
            "problem_044" => run_problem(problems::problem_044::problem_044, arg),   
            "problem_045" => run_problem(problems::problem_045::problem_045, arg),   
            "problem_046" => run_problem(problems::problem_046::problem_046, arg),   
            "problem_047" => run_problem(problems::problem_047::problem_047, arg),   
            "problem_148" => run_problem(problems::problem_148::problem_148, arg),             
    		_ => println!("Haven't done {} yet",arg)

    	}

    }

}