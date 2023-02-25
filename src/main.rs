use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Data {
    data: Vec<Vec<String>>,
}

struct Result {
    value: String,
    score: u64,
}

struct Handler {
    data: Data,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let split = msg.content.split(" ").collect::<Vec<&str>>();
        let command = split[0];
        let rest = &split[1..];

        if command == "lph" {
            let mut collected = Vec::<Result>::new();

            for d in &self.data.data {
                let mut score = 0;
                let i = 0;

                for s in rest.into_iter() {
                    for i in i..d.len() {
                        if d[i] == *s {
                            score += 1;
                            break;
                        }
                    }
                }

                collected.push(Result {
                    value: d.join(" "),
                    score,
                });
            }

            let mut results = Vec::from_iter(collected.iter().filter(|x| x.score > 0));
            results.sort_by(|a, b| b.score.cmp(&a.score));

            if let Err(why) = msg
                .channel_id
                .say(
                    &ctx.http,
                    Vec::from_iter(results.iter().map(|x| x.value.as_str()))[0..5].join("\n"),
                )
                .await
            {
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

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            data: serde_json::from_str(fs::read_to_string("./data.json").unwrap().as_str())
                .unwrap(),
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
