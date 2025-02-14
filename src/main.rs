/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::io::{self, Write};

use serenity::{
     async_trait,
     model::{channel::Message, gateway::Ready},
     prelude::*,
 };
use sled::Db;
 
impl DiscordToken {
    fn new(token: String) -> Self {
        let db = sled::open("token_new.db").expect("Failed to open database");
        DiscordToken { token, db }
    }

    fn save_token(&self) {
        self.db.insert("token", self.token.as_bytes()).expect("Failed to save Discord Token");
        self.db.flush().expect("failed to flush database");
    }

    fn get_token(&self) ->Option<String> {
        match self.db.get("discord_token").expect("Failed to get token") {
            Some(ivec) => Some(String::from_utf8(ivec.to_vec()).expect("Failed to convert token to string")),
            None => { println!("Token not found in database!");
                      None
            },
        }
    }
 }

const HELP_MESSAGE: &str = "
Hello Human!
I am a bunch of prewritten text that gets spewn out when you type `!help` in the chat.
I am not a real person, but I can help you with some basic commands.
Here are some things you can ask me:
- `!help` - I will tell you what I can do.
- `!info` - I will give you some information about myself.
- `!commands` - I will list all the commands I can respond to.
- `!ping` - I will respond with 'Pong!' to let you know I'm here.
- `!status` - I will tell you my current status.
- `!uptime` - I will tell you how long I've been running.
- `!version` - I will tell you my current version.
- `!invite` - I will give you a link to invite me to your server.
- `!feedback` - I will give you a link to provide feedback about me.

I hope that resolves your issue!

— Wabtec Bot 🤖
";
 
const INFO_MESSAGE: &str = "I am a Discord bot created to help you with various commands.";
const COMMANDS_MESSAGE: &str = "Available commands: !help, !info, !commands, !ping, !status, !uptime, !version, !invite, !feedback";
const PING_MESSAGE: &str = "Pong!";
const STATUS_MESSAGE: &str = "I am currently online and ready to assist you.";
const UPTIME_MESSAGE: &str = "I have been running for X hours.";
const VERSION_MESSAGE: &str = "I am running version 1.0.0.";
const INVITE_MESSAGE: &str = "Invite me to your server using this link: [invite link]";
const FEEDBACK_MESSAGE: &str = "Provide feedback about me using this link: [feedback link]";

 const HELP_COMMAND: &str = "!help";
 const INFO_COMMAND: &str = "!info";
 const LIST_COMMAND: &str = "!commands";
 const PING_COMMAND: &str = "!ping";
 const STATUS_COMMAND: &str = "!status";
 const UPTIME_COMMAND: &str = "!uptime";
 const VERSION_COMMAND: &str = "!version";
 const INVITE_COMMAND: &str = "!invite";
 const FEEDBACK_COMMAND: &str = "!feedback";

 struct DiscordToken {
    token: String,
    db: Db
 }

 struct Handler;
 
 #[async_trait]
 impl EventHandler for Handler {
     async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with('!') {
            let response = match msg.content.as_str() {
                HELP_COMMAND => HELP_MESSAGE,
                INFO_COMMAND => INFO_MESSAGE,
                LIST_COMMAND => COMMANDS_MESSAGE,
                PING_COMMAND => PING_MESSAGE,
                STATUS_COMMAND => STATUS_MESSAGE,
                UPTIME_COMMAND => UPTIME_MESSAGE,
                VERSION_COMMAND => VERSION_MESSAGE,
                INVITE_COMMAND => INVITE_MESSAGE,
                FEEDBACK_COMMAND => FEEDBACK_MESSAGE,
                _ => "Unknown command. Type !help for a list of available commands.",
            };
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
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
    let db = sled::open("token.db").expect("Failed to open database");
    let discord_token = DiscordToken {token: String::new(), db };

    /* let token = match discord_token.get_token() {
        Some(token) => token,
        None => {
            println!("Discord token not found in db! Please enter new token to save to db");
            let token = rpassword::read_password().expect("Failed to read token");
            let discord_token = DiscordToken::new(token.clone());
            discord_token.save_token();
            token
        }
    }; */
    let token = match discord_token.get_token() {
        Some(token) => token,
        None => {
            println!("Discord token not found in db! Please enter new token to save to db");
            print!("Enter Discord token: ");
            io::stdout().flush().unwrap();
            let mut token = String::new();
            io::stdin().read_line(&mut token).expect("Failed to read token");
            let token = token.trim().to_string();
            let discord_token = DiscordToken::new(token.clone());
            discord_token.save_token();
            token
        }
    };
 
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    
     if let Err(why) = client.start().await {
         println!("Client error: {:?}", why);
     }
 }