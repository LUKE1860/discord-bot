/* 
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "%ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content=="%end"{
        if let Err(why)=msg.channel_id.say(ctx.http,"Welcome!").await{
        println!("Error sending message:{:?}",why);
        }
        }
    }
    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
const TOKEN:&str="OTkzNzkyNTkxMTc2OTMzMzk3.GLgILp.2qSo0xDzLQwqarM-2cMAao36kx4kbxpeeMqJaY";
#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::DIRECT_MESSAGES;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&TOKEN, intents).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
if message.content=="@ping"{
if let Err(why)=message.channel_id.say(ctx.http,message.id.created_at()).await{
            println!("Error sending message:{:?}",why);
}
message.pinned
message.author.id.created_at()
message.link()
message.author.name
}

use serenity::async_trait;
use serenity::client::EventHandler;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::gateway::GatewayIntents;
use serenity::prelude::*;
use serenity::model::user::User;
struct Handler;
#[async_trait]
impl EventHandler for Handler{
async fn message(&self, ctx: Context, message:Message) {
if message.content=="@ping"{
if let Err(why)=message.channel_id.say(ctx.http,message.author.name).await{
            println!("Error sending message:{:?}",why);
}
}
}
async fn ready(&self,_:Context,ready:Ready){
println!("{} is connected",ready.user.name);
}
}
const TOKEN:&str="OTkzNzkyNTkxMTc2OTMzMzk3.GLgILp.2qSo0xDzLQwqarM-2cMAao36kx4kbxpeeMqJaY";
#[tokio::main]
async fn main(){
let intents=GatewayIntents::DIRECT_MESSAGES|GatewayIntents::GUILD_BANS|GatewayIntents::GUILD_MEMBERS|GatewayIntents::MESSAGE_CONTENT
|GatewayIntents::GUILD_MESSAGES;
let mut client=Client::builder(&TOKEN,intents).event_handler(Handler).await.expect("error!");
if let Err(why)=client.start().await{
println!("Client Error:{:?}",why);
}
}

//good one
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework, Command};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::model::guild::Ban;

#[command]
async fn hello(ctx:&Context,msg:&Message)-> CommandResult{
let a=msg.clone();
let b=a.author.name.as_str();
let mut c=String::from("Hello ");
c.push_str(b);
msg.channel_id.say(&ctx.http,c).await?;
Ok(())
}
#[command]
async fn ping(ctx:&Context,msg:&Message)->CommandResult{
msg.channel_id.say(&ctx.http,msg.author.id.created_at()).await?;
Ok(())
}
#[group]
#[commands(hello,ping)]
struct General;
struct Handler;
impl EventHandler for Handler{

}
const TOKEN:&str="OTkzNzkyNTkxMTc2OTMzMzk3.GLgILp.2qSo0xDzLQwqarM-2cMAao36kx4kbxpeeMqJaY";
#[tokio::main]
async fn main(){
let framework=StandardFramework::new()
.configure(|c|c.prefix("@")).group(&GENERAL_GROUP);
let intents=GatewayIntents::DIRECT_MESSAGES|GatewayIntents::GUILD_BANS|GatewayIntents::GUILD_MEMBERS|GatewayIntents::MESSAGE_CONTENT
|GatewayIntents::GUILD_MESSAGES;
let mut client=Client::builder(&TOKEN,intents).event_handler(Handler).framework(framework).await.expect("error!");
if let Err(why)=client.start().await{
println!("Client Error:{:?}",why);
}
}
guild_id.ban(ctx.http,,10)
//hooks are used for appending a function after command
#[hook]
async fn hello(ctx:&Context,msg:&Message,st:&str)-> CommandResult{
let a=msg.clone();
let b=a.author.name.as_str();
let mut c=String::from("Hello ");
c.push_str(b);
msg.channel_id.say(&ctx.http,c).await?;
Ok(())
}
ban(&ctx.http,663373499942830081,4).await?.expect("Error!");
let _ = GuildId(81384788765712384).ban(&http, user, 4).await;
//important
msg.channel_id.say(&ctx.http,args.parse::<String>().unwrap()).await?;
//fixing!
let mut a=async {args.parse::<usize>().unwrap()}; 
a.then(|x|async move {
for _ in 0..=x{
    msg.delete(&ctx.http).await.and_then();
}
}).await;
let cache=ctx.cache.current_user_id();
msg.channel_id.say(&ctx.http,cache.name).await?;
*/
mod env;
use serenity::framework::standard::macros::{command, group, hook};
use serenity::framework::standard::{CommandResult, StandardFramework, Command, Args, Delimiter, CommonOptions};
use serenity::model::channel::Message;
use serenity::model::webhook::Webhook;
use serenity::prelude::*;
use serenity::model::id::{GuildId, UserId,ChannelId, MessageId};
use serenity::model::channel::EmbedField;
use serenity::utils::Colour;
#[command]
async fn ban(ctx:&Context,msg:&Message,args:Args)->CommandResult{
    //let mut webhook:Webhook;
    let a=args.parse::<u64>().unwrap();
    let b=UserId(a);
    let c=msg.guild_id;
    let d=c.unwrap();
    //GuildId(81384788765712384).ban(&ctx.http, b, 4).await?;
   d.ban(&ctx.http,b,4).await?;  
    Ok(())
}
#[command]
async fn unban(ctx:&Context,msg:&Message,args:Args)->CommandResult{
    let a=args.parse::<u64>().unwrap();
    let b=UserId(a);
    let c=msg.guild_id;
    let d=c.unwrap();
    d.unban(&ctx.http,b).await.unwrap();
    Ok(())
}
#[command]
//dynamic channel id and http
async fn clear(ctx:&Context,msg:&Message,args:Args)->CommandResult{
let a=args.parse::<u64>().unwrap();
let b=msg.channel_id;
let c=msg.id;
let e:u64=b.try_into().unwrap();
let messages =b.messages(&ctx.http, |retriever| retriever.before(c).limit(a)).await?;
b.delete_messages(&ctx.http, messages).await?;
Ok(())
}
#[command]
// one cached user
//unwrap erases some yeeeeey
async fn info(ctx:&Context,msg:&Message)->CommandResult{
let c=msg.channel_id;
let j=msg.author.static_avatar_url().unwrap();
c.send_message(&ctx.http, |m| {
        m.embed(|e| {
        let a=msg.author.id.as_u64();
        let color=Colour::BLUE;
        let d=msg.guild_id;
        let g=d.unwrap();
        let h=g.as_u64();
        e.image(j);
        e.field("User_id",a,true);
        e.field("Guild_id",h,true);
        e.colour(color)
})
}).await?;
Ok(())
}
#[command]
async fn kick(ctx:&Context,msg:&Message,args:Args)->CommandResult{
let c=args.parse::<u64>().unwrap();
let a=msg.guild_id;
let b=a.unwrap();
b.kick(&ctx.http,c).await?;
Ok(())
}
#[command]
//write a command for mute and warn
//edit_member in GuildId
async fn mute(ctx:&Context,msg:&Message,args:Args)->CommandResult{
let a=args.parse::<u64>().unwrap();
let b=msg.member(&ctx.http).await?;

Ok(())
}
#[command]
async fn hello(ctx:&Context,msg:&Message)->CommandResult{
msg.channel_id.say(&ctx.http, "\u{0058}\u{0044}").await?;
Ok(())
}
#[group]
#[commands(ban,unban,clear,info,kick,mute,hello)]
struct General;
struct Handler;
impl EventHandler for Handler{}
#[tokio::main]
async fn main(){
let a=env::EnvGet::setup().env().unwrap();
let token=a.trim();
let framework=StandardFramework::new()
.configure(|c|c.prefix("%")).group(&GENERAL_GROUP);
let intents=GatewayIntents::DIRECT_MESSAGES|GatewayIntents::GUILD_BANS|GatewayIntents::GUILD_MEMBERS|GatewayIntents::MESSAGE_CONTENT
|GatewayIntents::GUILD_MESSAGES|GatewayIntents::GUILDS;
let mut client=Client::builder(&token,intents).event_handler(Handler).framework(framework).await.expect("error!");
if let Err(why)=client.start().await{
println!("Client Error:{:?}",why);
}
}