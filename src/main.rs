use std::fs;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serde_json::{Value, json};

struct Handler {
    data: Value
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let split = msg.content.split(" ").collect::<Vec<&str>>();
        let command = split[0];
        let rest = &split[1..];

        if command == "lph" {
            let mut collected = Vec::<Value>::new();

            for d in self.data["data"].as_array().unwrap() {
                let mut score = 0;
                let i = 0u64;

                for s in rest.into_iter() {
                    let words = d.as_array().unwrap();

                    for i in i..(words.len() as u64) {
                        if words[i as usize].as_str().unwrap() == *s {
                            score += 1;
                            break;
                        }
                    }
                }

                collected.push(json!({
                    "value": Vec::from_iter(d.as_array().unwrap().iter().map(|x| x.as_str().unwrap())).join(" "),
                    "score": score
                }));
            }

            let mut results = Vec::from_iter(collected.iter().filter(|x| x["score"].as_u64().unwrap() > 0));
            results.sort_by(|a, b| b["score"].as_u64().unwrap().cmp(&a["score"].as_u64().unwrap()));

            if let Err(why) = msg.channel_id.say(&ctx.http, Vec::from_iter(results.iter().map(|x| x["value"].as_str().unwrap()))[0..5].join("\n")).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = "NTQ3MzEwNzY4ODYxODcyMTcy.G5jY6M.LXAsnN4xjDHb9Pgn_HlKuIAxwhxn5jhWx8c7TU";
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler { data: serde_json::from_str(fs::read_to_string("./data.json").unwrap().as_str()).unwrap() }).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}