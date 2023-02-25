use serde::{Deserialize, Serialize};
use serde_json;
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
    score: u32,
}

struct Handler {
    data: Data,
}

fn calculate_score(input: &[&str], words: &Vec<String>) -> u32 {
    let mut score: u32 = 0;
    let i = 0usize;

    for s in input.into_iter() {
        for i in i..words.len() {
            if words[i] == *s {
                score += 1;
                break;
            }
        }
    }

    return score;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let split = msg.content.split(" ").collect::<Vec<&str>>();
        let command = split[0];
        let rest = &split[1..];

        if command == "lph" {
        } else if command == "lph-stats" {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("Messages: {}", self.data.data.len()))
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        } else if command == "lph-search" {
            let now = std::time::Instant::now();
            let mut collected = Vec::<Result>::new();

            for words in &self.data.data {
                let score = calculate_score(rest, words);

                collected.push(Result {
                    value: words.join(" "),
                    score,
                });
            }

            let mut results = Vec::from_iter(collected.iter().filter(|x| x.score > 0));
            results.sort_by(|a, b| b.score.cmp(&a.score));

            let reply = format!(
                "{} results ({:.2} seconds)\n\n{}",
                results.len(),
                now.elapsed().as_secs_f32(),
                Vec::from_iter(results.iter().map(|x| x.value.as_str()))[0..10].join("\n")
            );

            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, &reply.get(0..2000).unwrap_or(&reply))
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
