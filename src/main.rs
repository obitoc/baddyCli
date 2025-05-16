// This is the First Project in Rust
// This code made by for GREPING OUT
// if you are a baddy dm me @x9639 in telegram
// Enjoy i guess
// Importing Env from Standerd library
use std::{env, fs};
// Importing Some Color
use colored::{Color,Colorize};
mod Ai;
fn Greper(args: &Vec<String>) {

   // If The lovley user dont provide the needed Arguments

   if args.len() < 4 {

       eprintln!("Usage: baddyCli <fileName> <Word>\nLove U.");
       std::process::exit(1);
   }
   // Get Content Off The File
   let Content = fs::read_to_string(&args[2]).expect("FuckOff");
   // For loop for Each Line
   for (lineNum,line) in Content.lines().enumerate() {
       // Seeing if the word of yours in the Line ( btw you look cute )
       if line.contains(&args[3]) {
           // Do you support Femboys ?
           println!("[{}] : {}",lineNum,line.replace(&args[3],&args[3].color(Color::BrightCyan).to_string()));
       }
   }
   // Printing Cute msg
   println!("Done. Love you babe");
}
fn HelpFunc(){
    println!("Usage: baddyCli <mode>\n\nGreper:\nbaddyCli -grep <fileName> <word>\nAi:\nbaddyCli -ai <prompt>\nHelp:\nbaddyCli -h\nLove U.");
}
enum ParseModes {
    AiMode,
    HelpMode,
    GreperMode,
}
fn ParseMode(input: &str) -> ParseModes {
    match input {
        "-ai" => ParseModes::AiMode,
        "-h" => ParseModes::HelpMode,
        "-grep" => ParseModes::GreperMode,
        _ => ParseModes::HelpMode,
    }
}
fn main() {
   // Geting Args
   let args: Vec<String> = env::args().collect();
   if args.len() < 2 {

       eprintln!("Usage: baddyCli <mode>\n\nGreper:\nbaddyCli -grep <fileName> <word>\nAi:\nbaddyCli -ai <prompt>\nHelp:\nbaddyCli -h\nLove U.");
       std::process::exit(1);
   }
   let TypeOf = ParseMode(&args[1]);
   match TypeOf {
       ParseModes::HelpMode => {
           HelpFunc();
       },
       ParseModes::GreperMode => {
           Greper(&args);
       },
       ParseModes::AiMode => {
           Ai::AiMod(&args);
       },
   }
}

// Code is cute and me too
