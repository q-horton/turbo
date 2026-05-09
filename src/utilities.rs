use std::env;
use std::time::Duration;
use std::sync::Arc;
use serde_json::Value;

use serenity::model::channel::*;
use poise::serenity_prelude as serenity;
use serenity::{ChannelId, MessageId, GuildId};
use serenity::builder::{CreateMessage, CreateAttachment};

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

const MAX_EMOJI_FILE_SIZE: u32 = 256_000;
const DAY_IN_SEC: u64 = 86400;

/// Votey Thumbs
///
/// A completely original idea
pub async fn voty(ctx: &Context<'_>, msg: &serenity::Message) -> Result<(), Error> {
    let thumbsup = ReactionType::Unicode("👍".to_string());
    let thumbsdown = ReactionType::Unicode("👎".to_string());

    msg.react(ctx, thumbsup).await?;
    msg.react(ctx, thumbsdown).await?;
    Ok(())
}

/// Memberships command
///
/// Pulls current membership count and quorum, returning them as a string.
pub async fn memberships() -> Result<String, reqwest::Error> {
    let sheet_id = env::var("MEMBER_STATS_SHEET_ID").expect("Expected a google sheet ID in the environment");
    let cell_id = env::var("MEMBER_STATS_CELL").expect("Expected a google sheet cell ID in the environment");
    let sheets_api = env::var("GOOGLE_SHEETS_API_KEY").expect("Expected a google sheets API key in the environment");
    let membership_url: String = format!("https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}", sheet_id, cell_id, sheets_api);
    let resp = reqwest::get(membership_url)
        .await?
        .text()
        .await?;

    let parsed: Value = serde_json::from_str(&resp).unwrap();

    // Access fields using square brackets
    let val_a = &parsed["values"][0].get(0).unwrap();
    let members: i32 = val_a.as_str().unwrap().parse().unwrap();
    let val_b = &parsed["values"][1].get(0).unwrap();
    let quorum: i32 = val_b.as_str().unwrap().parse().unwrap();

    let msg: String = format!("There are currently {members} members of UQ MARS, making the current quorum {quorum}");
    println!("{msg}");
    Ok(msg)
}

fn is_valid_emoji_name(name: &str) -> bool {
    for c in name.bytes() {
        if !(c.is_ascii_alphanumeric() || c == '_'.to_ascii_lowercase() as u8) {
            return false;
        }
    }
    true
}

fn check_attachment(img: &serenity::Attachment) -> Option<String> {
    if img.size >= MAX_EMOJI_FILE_SIZE {
        return Some("Image is too large".to_string());
    }
    match img.dimensions() {
        Some((x, y)) => {
            if x != y {
                return Some("Image is not square".to_string());
            }
        },
        None => {
            return Some("File is not an image".to_string());
        }
    }
    None
}

async fn vote_counting(
    http: Arc<serenity::Http>,
    chann_id: ChannelId,
    msg_id: MessageId,
    guild: GuildId,
    img: CreateAttachment,
    emoji_name: &str
) {
    let message = chann_id.message(&http, msg_id).await.unwrap();
    let mut yays: u64 = 0;
    let mut nays: u64 = 0;
    for reaction in &message.reactions {
        match reaction.reaction_type.to_string().chars().next().unwrap() {
            '👍' => yays = reaction.count,
            '👎' => nays = reaction.count,
            _ => ()
        }
    }

    let emoji: Option<serenity::Emoji>;
    let response = format!("The votes are in: {}.", if yays > nays {
        let img_bytes = img.to_base64();
        emoji = Some(guild.create_emoji(&http, emoji_name, &img_bytes).await.unwrap());
        "yay"
    } else {
        emoji = None;
        "nay"
    });

    let result_msg = chann_id.send_message(&http, CreateMessage::new()
            .reference_message((chann_id, msg_id))
            .content(response)
        ).await.unwrap();

    if let Some(x) = emoji{
        result_msg.react(&http, ReactionType::Custom {
            animated: x.animated,
            id: x.id,
            name: Some(x.name) }).await.unwrap();
    }
}

pub async fn emoji_vote(
    ctx: Context<'_>,
    img: serenity::Attachment,
    emoji_name: String,
    time_days: u64
) -> Result<(), Error> {
    let img_url = &img.url;
    if !is_valid_emoji_name(&emoji_name) {
        ctx.say("Invalid emoji name").await?;
        return Ok(());
    }
    if let Some(resp) = check_attachment(&img) {
        ctx.say(resp).await?;
        return Ok(());
    }
    let msg_txt: String = format!("`:{}:`, yay or nay?", emoji_name);
    let img_attachment = serenity::CreateAttachment::url(&ctx, img_url).await.unwrap();
    let img_attachment_dup = img_attachment.clone();
    let msg_reply = poise::CreateReply::default()
        .content(msg_txt)
        .attachment(img_attachment);
    let msg_sent = ctx.send(msg_reply).await?;
    let msg = msg_sent.message().await?;
    voty(&ctx, &msg).await?;

    let http = ctx.serenity_context().http.clone();
    let chann_id = ctx.channel_id();
    let msg_id = msg.id;
    let guild = ctx.guild_id().unwrap();

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(time_days * DAY_IN_SEC)).await;
        vote_counting(http, chann_id, msg_id, guild, img_attachment_dup, &emoji_name).await;
    });

    Ok(())
}
