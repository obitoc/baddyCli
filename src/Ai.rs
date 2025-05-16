// Some Imports
use reqwest::blocking::Client;
use reqwest::header;
use serde_json::{json,Value};
pub fn AiMod(args: &Vec<String>) {
    // New Client
    let cli = Client::new();
    // New Headers Map
    let mut Headers = header::HeaderMap::new();
    // Checking Some cute stuff
   if args.len() < 3 {

       eprintln!("Usage: baddyCli -ai <prompt>\nLove U.");
       std::process::exit(1);
   }
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
       let headName = header::HeaderName::from_bytes(key.as_bytes()).unwrap();
       let headVal = header::HeaderValue::from_str(value).unwrap();
       Headers.insert(headName,headVal);

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
  let res = cli.post("https://www.blackbox.ai/api/chat").headers(Headers).json(&json_data).timeout(std::time::Duration::from_secs(80)).send().expect("idk A nigger. ");
  println!("{}",res.text().unwrap());
}

// This source created By Obito
// You need to be cute to edit
// this V1.1 update has one validetor every body use
// in Next update you Will gona be eable to Put your Oun Valid Token
// Enjoy .
