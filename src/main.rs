// This is the First Project in Rust
// This code made For A nice Cli Tool
// if you are a baddy dm me @x9639 in telegram
// Enjoy i guess
// Importing Env from Standerd library
use std::env;
// Importing Some Color
mod utils;
use utils::{AiMod,Greper,HelpFunc,Infor};
enum ParseModes {
    AiMode,
    HelpMode,
    GreperMode,
    InfoMode,
}
fn ParseMode(input: &str) -> ParseModes {
    match input {
        "-ai" => ParseModes::AiMode,
        "-h" => ParseModes::HelpMode,
        "-grep" => ParseModes::GreperMode,
        "-info" => ParseModes::InfoMode,
        _ => ParseModes::HelpMode,
    }
}
fn main() {
   // Geting Args
   let args: Vec<String> = env::args().collect();
   if args.len() < 2 {

       eprintln!("Usage: baddyCli <mode>\n\nGreper:\nbaddyCli -grep <fileName> <word>\nAi:\nbaddyCli -ai <prompt>\nInfo About device :\nbaddyCli -info\nHelp:\nbaddyCli -h\nLove U.");
       std::process::exit(1);
   }
   let TypeOf: ParseModes = ParseMode(&args[1]);
   match TypeOf {
       ParseModes::HelpMode => {
           HelpFunc();
       },
       ParseModes::GreperMode => {
           Greper(&args);
       },
       ParseModes::AiMode => {
           AiMod(&args);
       },
       ParseModes::InfoMode => {
           Infor();

       }
   }
}

// Code is cute and me too
