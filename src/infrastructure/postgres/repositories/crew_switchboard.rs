use anyhow::Result;
use std::sync::Arc;

use axum::async_trait;

use crate::{
    domain::{
        repositories::crew_switchboard::CrewSwitchBoardRepository,
        value_objects::quest_adventurer_junction::QuestAdventurerJunction,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct CrewSwitchboardPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewSwitchboardPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CrewSwitchBoardRepository for CrewSwitchboardPostgres {
    async fn join(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        unimplemented!("Method not implemented yet")
    }

    async fn leave(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        unimplemented!("Method not implemented yet")
    }
}
