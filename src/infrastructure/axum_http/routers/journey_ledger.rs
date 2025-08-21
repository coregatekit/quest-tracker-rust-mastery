use std::sync::Arc;

use axum::{
    Extension, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::patch,
};

use crate::{
    application::usecases::journey_ledger::JourneyLedgersUseCase,
    domain::repositories::{
        journey_ledger::JourneyLedgerRepository, quest_viewing::QuestViewingRepository,
    },
    infrastructure::{
        axum_http::middlewares::guild_commanders_authorization,
        postgres::{
            postgres_connection::PgPoolSquad,
            repositories::{
                journey_ledger::JourneyLedgerPostgres, quest_viewing::QuestViewingPostgres,
            },
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let journey_ledger_repository = JourneyLedgerPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let journey_ledger_usecase = JourneyLedgersUseCase::new(
        Arc::new(journey_ledger_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/in-journey/:quest_id", patch(in_journey))
        .route("/to-completed/:quest_id", patch(to_completed))
        .route("/to-failed/:quest_id", patch(to_failed))
        .route_layer(middleware::from_fn(guild_commanders_authorization))
        .with_state(Arc::new(journey_ledger_usecase))
}

pub async fn in_journey<T1, T2>(
    State(journey_ledger_usecase): State<Arc<JourneyLedgersUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match journey_ledger_usecase
        .in_journey(quest_id, guild_commander_id)
        .await
    {
        Ok(result) => (
            StatusCode::OK,
            format!("Quest {} is now in journey", result),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to put quest in journey: {}", e),
        )
            .into_response(),
    }
}

pub async fn to_completed<T1, T2>(
    State(journey_ledger_usecase): State<Arc<JourneyLedgersUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match journey_ledger_usecase
        .to_completed(quest_id, guild_commander_id)
        .await
    {
        Ok(result) => {
            (StatusCode::OK, format!("Quest {} is now completed", result)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to put quest in completed: {}", e),
        )
            .into_response(),
    }
}

pub async fn to_failed<T1, T2>(
    State(journey_ledger_usecase): State<Arc<JourneyLedgersUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match journey_ledger_usecase
        .to_failed(quest_id, guild_commander_id)
        .await
    {
        Ok(result) => (StatusCode::OK, format!("Quest {} is now failed", result)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to put quest in failed: {}", e),
        )
            .into_response(),
    }
}
