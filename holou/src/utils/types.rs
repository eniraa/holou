use std::sync::Arc;

use chrono::prelude::*;
use poise::serenity_prelude as serenity;
use serenity::prelude::*;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<serenity::ShardManager>;
}

pub struct StartTimeContainer;

impl TypeMapKey for StartTimeContainer {
    type Value = DateTime<Utc>;
}
