// Some Imports
use reqwest::blocking::Client;
use sysinfo::System;
use reqwest::header;
use serde_json::{json,Value};
use std::fs;
use colored::{Color,Colorize};
pub fn greper(args: &[String]) {

   // If The lovley user dont provide the needed Arguments

   if args.len() < 4 {

       eprintln!("Usage: baddyCli <fileName> <Word>\nLove U.");
       std::process::exit(1);
   }
   let word = &args[3];
   let file_name = &args[2];
   // Get Content Off The File

   let content = fs::read_to_string(file_name).expect("FuckOff");
   // For loop for Each Line
   for (line_num,line) in content.lines().enumerate() {
       // Seeing if the word of yours in the Line ( btw you look cute )
       if line.contains(word) {
           // Do you support Femboys ?

           println!("{} >  {}",line_num.to_string().truecolor(150,95,255),line.replace(&args[3],&args[3].color(Color::BrightCyan).to_string()).bold());
       }
   }
   // Printing Cute msg
   println!("Done. Love you babe");
}

pub fn helpfunc(){
    println!("Usage: baddyCli <mode>\n\nGreper:\nbaddyCli -grep <fileName> <word>\nAi:\nbaddyCli -ai <prompt>\nDevice inf:\nbaddyCli -info\nHelp:\nbaddyCli -h\nLove U.");
}
pub fn ai_mod(args: &[String]) {

    // Checking Some cute stuff
   if args.len() < 3 {

       eprintln!("Usage: baddyCli -ai <prompt>\nLove U.");
       std::process::exit(1);
   }

    // New Client
    let cli = Client::new();
    // New Headers Map
    let mut headers = header::HeaderMap::new();


   // Some Headers
   let heades = [
       ("authority","www.blackbox.ai"),
       ("accept","*/*"),
       ("User-Agent","Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Mobile Safari/537.36"),
       ("content-type","application/json"),
       ("origin","https://www.blackbox.ai"),
       ("referer","https://www.blackbox.ai"),
   ];
   // For loop and appending to Headers Map
   for (key,value) in heades {
       let headname = header::HeaderName::from_bytes(key.as_bytes()).unwrap();
       let headval = header::HeaderValue::from_str(value).unwrap();
       headers.insert(headname,headval);

   }
   // Json Data
   let json_data = json!({
  "messages": [
    {
      "role": "user",
      "content": &args[2],
      "id": ""
    }
  ],
  "agentMode": {},
  "id": "",
  "previewToken": None::<Value>,
  "userId": None::<Value>,
  "codeModelMode": true,
  "trendingAgentMode": {},
  "isMicMode": false,
  "userSystemPrompt": None::<Value>,
  "maxTokens": 1024,
  "playgroundTopP": None::<Value>,
  "playgroundTemperature": None::<Value>,
  "isChromeExt": false,
  "githubToken": "",
  "clickedAnswer2": false,
  "clickedAnswer3": false,
  "clickedForceWebSearch": false,
  "visitFromDelta": false,

  "isMemoryEnabled": false,
  "mobileClient": false,
  "userSelectedModel": None::<Value>,
  "validated": "00f37b34-a166-4efb-bce5-1312d87f2f94",
  "imageGenerationMode": false,
  "webSearchModePrompt": false,
  "deepSearchMode": false,
  "domains": None::<Value>,
  "vscodeClient": false,
  "codeInterpreterMode": false,
  "customProfile": {
    "name": "",
    "occupation": "",
    "traits": [],
    "additionalInfo": "",
    "enableNewChats": false
  },
  "webSearchModeOption": {
    "autoMode": true,
    "webMode": false,
    "offlineMode": false
  },
  "session": None::<Value>,
  "isPremium": false,
  "subscriptionCache": None::<Value>,
  "beastMode": false,
  "reasoningMode": false,
  "designerMode": false,
  "workspaceId": ""

   });
   // fetching Out
  let res = cli.post("https://www.blackbox.ai/api/chat").headers(headers).json(&json_data).timeout(std::time::Duration::from_secs(80)).send().expect("idk A nigger. ");
  println!("{}",res.text().unwrap());
}
pub fn inform(){
    //  Cute Formating
    let formated = format!("

    ______________________________________________

     OS Version > {}
     Host Name > {}
     Device Name > {}
     Kernel Version > {}

    ______________________________________________

    ",System::os_version().unwrap(),System::host_name().unwrap(),System::name().unwrap(),System::kernel_version().unwrap());
    // print out
    println!("{}",formated.bold());
  }
