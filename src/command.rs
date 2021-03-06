use std::string::String;

use discord::model::{Message, ChannelId, User};

#[derive(Debug)]
pub struct Command {
    user: User,
    channel_id: ChannelId,
    command: String,
    args: Vec<String>,
}

impl Command {
    pub fn parse(message: &Message) -> Command {
        let mut args: Vec<String> = message.content.split_whitespace().map(|s| String::from(s)).collect();
        Command {
            user: message.author.clone(),
            channel_id: message.channel_id.clone(),
            command: args.remove(0),
            args: args,
        }
    }
    
    pub fn user(&self) -> &User {
        &self.user
    }
    pub fn channel_id(&self) -> &ChannelId {
        &self.channel_id
    }
    pub fn command(&self) -> &str {
        &*self.command
    }
    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}
