use anyhow::Result;
use std::sync::Arc;

use crate::domain::{
    entities::guild_commanders::RegisterGuildCommanderEntity,
    repositories::guild_commanders::GuildCommandersRepository,
};

pub struct GuildCommandersUsesCase<T>
where
    T: GuildCommandersRepository + Send + Sync,
{
    guild_commanders_repository: Arc<T>,
}

impl<T> GuildCommandersUsesCase<T>
where
    T: GuildCommandersRepository + Send + Sync,
{
    pub fn new(guild_commanders_repository: Arc<T>) -> Self {
        Self {
            guild_commanders_repository,
        }
    }

    pub async fn register(
        &self,
        register_guild_commander_entity: RegisterGuildCommanderEntity,
    ) -> Result<i32> {
        unimplemented!("Register method not implemented yet")
    }
}
