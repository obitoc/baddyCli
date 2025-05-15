// This is the First Project in Rust
// This code made for Cute ones
// if you are a baddy dm me @x9639 in telegram
// Enjoy i guess
// Importing Env from Standerd library
use std::{env, fs};
// Importing Some Color
use colored::{Color,Colorize};
fn main() {
   // Geting Args
   let args: Vec<String> = env::args().collect();
   // If The lovley user dont provide the needed Arguments
   if args.len() < 3 {
       eprintln!("Usage: baddyCli <fileName> <Word>\nLove U.");
       std::process::exit(1);
   }
   // Get Content Off The File
   let Content = fs::read_to_string(&args[1]).expect("FuckOff");
   // For loop for Each Line
   for (lineNum,line) in Content.lines().enumerate() {
       // Seeing if the word of yours in the Line ( btw you look cute )
       if line.contains(&args[2]) {
           // Do you support Femboys ?
           println!("[{}] : {}",lineNum,line.replace(&args[2],&args[2].color(Color::BrightCyan).to_string()));
       }
   }
   // Printing Cute msg
   println!("Done. Love you babe");

}
// Code is cute and me too
