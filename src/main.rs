mod env;
mod time;
use std::time::Duration;
use std::ops::Add;
use serenity::model::application::command::Command;
use serenity::framework::standard::macros::{command, group, hook};
use serenity::framework::standard::{Args,StandardFramework,CommandResult};
use serenity::model::Timestamp;
use serenity::model::channel::Message;
use serenity::model::prelude::CommandId;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::{prelude::*, async_trait};
use serenity::model::id::{GuildId, UserId,ChannelId, MessageId};
use serenity::utils::Colour;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
//TODO! ub on passing 24 hours
//ban someone specyfing 1. id of user 2.reason for it 3. delete messages from 0 to 7 days
#[command]
async fn ban(ctx:&Context,msg:&Message,mut args:Args)->CommandResult{
    let a=args.single::<u64>().unwrap();
    let e=args.single::<String>().unwrap();
    let d=args.single::<u8>().unwrap();
    let b=UserId(a);
    let c=msg.guild_id.unwrap();
    c.ban_with_reason(&ctx.http,b,d,e).await?;
    Ok(())
}
#[command]
async fn unban(ctx:&Context,msg:&Message,args:Args)->CommandResult{
    let a=args.parse::<u64>().unwrap();
    let b=UserId(a);
    let c=msg.guild_id.unwrap();
    c.unban(&ctx.http,b).await.unwrap();
    Ok(())
}
//clear x messages on channel and clear message
#[command]
async fn clear(ctx:&Context,msg:&Message,args:Args)->CommandResult{
let a=args.parse::<u64>().unwrap();
let b=msg.channel_id;
let c=msg.id;
let messages =b.messages(&ctx.http, |retriever| retriever.before(c).limit(a)).await?;
b.delete_messages(&ctx.http, messages).await?;
b.delete_message(&ctx.http,c).await?;
Ok(())
}
#[command]
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
//kick someone specyfing the reason
#[command]
async fn kick(ctx:&Context,msg:&Message,mut args:Args)->CommandResult{
let c=args.single::<u64>().unwrap();
let f=args.single::<String>().unwrap();
let mut l=String::from("Reason:");
l.push_str(f.trim());
let a=msg.guild_id;
let b=a.unwrap();
let id=UserId(c);
let s=id.create_dm_channel(&ctx.http).await?;
s.send_message(&ctx.http, |g|g.content(&f)).await?;
b.kick(&ctx.http,c).await?;
Ok(())
}
//mute someone specyfing 1.id of user 2.reason for mute 3.hours or minutes (h or m)
// i need to assemble a date and calculate hours to 24
#[command]
async fn mute(ctx:&Context,msg:&Message,mut args:Args)->CommandResult{
let a=args.single::<u64>().unwrap();
let b=args.single::<String>().unwrap();
let c=args.single::<String>().unwrap();
let d=format!("Reason:{}",b);
let id=UserId(a);
let s=id.create_dm_channel(&ctx.http).await?;
let guild=msg.guild_id.unwrap();
let time=Timestamp::now();
let l=time.date().to_string();
let duration=Duration::new(time::Parser::from(time).calculate(c),0);
println!("{:?}",duration);
let t=time.time().add(duration).to_string();
let h=format!("{l} {t}Z");
println!("{h}");
//println!("{}",time::Parser::from(time).calculate(String::from("20m")));
guild.edit_member(&ctx.http,id,|f|f.disable_communication_until(h)).await.unwrap();
s.send_message(&ctx.http,|m|m.content(&d)).await?;
Ok(())
}
//unmute someone specifing 1.id of user 2.reason for it
#[command]
async fn unmute(ctx:&Context,msg:&Message,mut args:Args)->CommandResult{
let a=args.single::<u64>().unwrap();
let b=args.single::<String>().unwrap();
let c=format!("Reason:{}",b);
let id=UserId(a);
let s=id.create_dm_channel(&ctx.http).await?;
let guild=msg.guild_id.unwrap();
s.send_message(&ctx.http, |l|l.content(&c)).await?;
guild.edit_member(&ctx.http,id,|f|f.enable_communication()).await?;
Ok(())
}
//developer command
#[command]
async fn time(ctx:&Context,msg:&Message)->CommandResult{
let time=Timestamp::now();
let f=format!("{}",time::Parser::from(time).convert());
msg.channel_id.say(&ctx.http,f).await?;
Ok(())
}
//lists bans
#[command]
async fn list_bans(ctx:&Context,msg:&Message)->CommandResult{
let m=msg.guild_id.unwrap().bans(&ctx.http).await?;
let s=m.get(0).unwrap();
let h=*s.to_owned().user.id.as_u64();
msg.channel_id.say(&ctx.http,&h).await?;
Ok(())
}
#[command]
async fn hello(ctx:&Context,msg:&Message)->CommandResult{
Command::create_global_application_command(&ctx.http,|s|{s.name("hello").description("xd")
.create_option(|d|d.name("string").description("give a name").kind(CommandOptionType::String).required(true))
}).await?;
Ok(())
}
#[group]
#[commands(ban,unban,clear,info,kick,mute,unmute,time,list_bans)]
struct General;
struct Handler;
#[async_trait]
impl EventHandler for Handler{
async fn interaction_create(&self,ctx:Context,interaction:Interaction){
if let Interaction::ApplicationCommand(command)=interaction{
if let Err(why) = command.create_interaction_response(&ctx.http,|response|
response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|message| message.content("Hello")))
.await
{
    println!("Cannot respond to slash command: {}", why);   
}
}
}

}
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