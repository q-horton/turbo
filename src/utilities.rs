use serenity::model::channel::*;
use poise::serenity_prelude as serenity;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

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
