use poise::serenity_prelude as serenity;

use crate::utils::types::*;

/// Commands for debugging and diagnostics.
#[poise::command(slash_command, subcommands("ping", "kill"))]
pub async fn debug(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Check the latency.
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let shard_man = data
        .get::<ShardManagerContainer>()
        .expect("Shard manager not found");

    let runners = shard_man.runners.lock().await;
    let runner = runners
        .get(&ctx.serenity_context().shard_id)
        .expect("Shard runner not found");

    poise::send_reply(
        ctx,
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::default()
                .title("Pong! 🏓")
                .description(format!(
                    "Shard latency is {}",
                    runner.latency.map_or(String::from("unknown"), |d| {
                        (d.subsec_micros() as f64 / 1000.0).to_string() + "ms"
                    })
                )),
        ),
    )
    .await?;
    Ok(())
}

/// Kill the bot.
#[poise::command(slash_command, owners_only)]
async fn kill(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let shard_man = data
        .get::<ShardManagerContainer>()
        .expect("Shard manager not found");

    poise::send_reply(
        ctx,
        poise::CreateReply::default()
            .embed(serenity::CreateEmbed::default().title("Shutting down… 🥀")),
    )
    .await?;
    shard_man.shutdown_all().await;
    Ok(())
}
