use crate::{
    entities::{apple_event::AppleEvent, onion::Onion, pear_event::PearEvent},
    use_cases::{
        get_onion::{GetOnion, GetOnionUseCase},
        update_onion::{ExpectedOldValues, OnionOptionalUpdates, UpdateOnion, UpdateOnionUseCase},
    },
    Result,
};
use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

const BATCH_SIZE: u64 = 100;

pub enum AppleEventKind {
    // Batch of kinds replaced with SomeKind
    SomeKind,
}

#[async_trait]
pub trait ProcessAppleEvent: Send + Sync {
    async fn process_apple_event(&mut self, event: AppleEvent, onion: Onion) -> Result<()>;

    fn get_apple_event_kind(&self, event: AppleEvent) -> Result<AppleEventKind>;
}

#[async_trait]
pub trait ProcessPearEvent: Send + Sync {
    async fn process_pear_event(&mut self, event: PearEvent, onion: Onion) -> Result<()>;
}

#[async_trait]
pub trait ListSequentialPearEvents: Send + Sync {
    async fn list_sequential_pear_events(
        &self,
        onion_id: Uuid,
        start_sequence_id: u64,
        batch_size: u64,
    ) -> Result<Vec<PearEvent>>;
}

#[async_trait]
pub trait ListSequentialAppleEvents: Send + Sync {
    async fn list_sequential_apple_events(
        &self,
        onion_id: Uuid,
        apple_id: String,
        start_sequence_id: u64,
        batch_size: u64,
    ) -> Result<Vec<AppleEvent>>;
}

pub struct ProcessNewEvents<'a, CCISR>
where
    CCISR: GetOnion + UpdateOnion,
{
    pub onion_repo: &'a mut CCISR,
    pub apple_event_repo: &'a mut dyn ListSequentialAppleEvents,
    pub pear_event_repo: &'a mut dyn ListSequentialPearEvents,
    pub apple_events_processor: &'a mut dyn ProcessAppleEvent,
    pub pear_events_processor: &'a mut dyn ProcessPearEvent,
}

impl<CCISR> ProcessNewEvents<'_, CCISR>
where
    CCISR: GetOnion + UpdateOnion,
{
    pub async fn execute(&mut self, onion_id: Uuid) -> Result<()> {
        loop {
            let onion = self.get_onion(onion_id).await?;

            let Some(onion) = onion else {
                // Some log
                return Ok(());
            };

            if !onion.ready_to_process_events() {
                // Some log
                return Ok(());
            }

            // some matching on what to process based on onion state
            // replaced with const
            let events_processed = 0;

            if events_processed < BATCH_SIZE {
                break;
            }
        }

        Ok(())
    }

    async fn get_onion(&mut self, onion_id: Uuid) -> Result<Option<Onion>> {
        GetOnionUseCase {
            onion_repo: self.onion_repo,
        }
        .execute(onion_id)
        .await
    }

    async fn process_new_apple_events(&mut self, apple_id: String, onion: &Onion) -> Result<u64> {
        let apple_events = self
            .apple_event_repo
            .list_sequential_apple_events(
                onion.onion_id,
                apple_id,
                onion
                    .apple_last_processed_sequence_id
                    .map(|id| id + 1)
                    .unwrap_or(0) as u64,
                BATCH_SIZE,
            )
            .await?;

        // Some log
        let event_count = apple_events.len();

        for event in apple_events {
            match self
                .process_new_apple_event_with_retries(&event, onion.clone())
                .await
            {
                Ok(_) => {
                    self.update_onion_for_last_apple_event_processed(&event, onion.onion_id)
                        .await
                }
                Err(e) => {
                    // Some log
                    self.block_onion(onion).await?;
                    Err(e)
                }
            }?
        }

        Ok(event_count as u64)
    }

    async fn process_new_apple_event_with_retries(
        &mut self,
        event: &AppleEvent,
        onion: Onion,
    ) -> Result<()> {
        // Some retry logic based on backoffs
        Ok(())
    }

    async fn update_onion_for_last_apple_event_processed(
        &mut self,
        event: &AppleEvent,
        apple_id: Uuid,
    ) -> Result<()> {
        use AppleEventKind::*;

        let updates = match self
            .apple_events_processor
            .get_apple_event_kind(event.clone())?
        {
            // Some logic of how we build OptionaUpdates
            SomeKind => OnionOptionalUpdates::default(),
        };

        UpdateOnionUseCase {
            onion_repository: self.onion_repo,
        }
        .execute(
            apple_id,
            updates,
            ExpectedOldValues {
                apple_id: Some(Some(event.apple_id.clone())),
            },
        )
        .await
    }

    async fn process_new_pear_events(&mut self, onion: &Onion) -> Result<u64> {
        let pear_events = self
            .pear_event_repo
            .list_sequential_pear_events(
                onion.onion_id,
                (onion.pear_last_processed_sequence_id + 1) as u64,
                BATCH_SIZE,
            )
            .await?;

        let event_count = pear_events.len();

        for event in pear_events {
            match self
                .process_new_pear_event_with_retries(&event, onion.clone())
                .await
            {
                Ok(_) => {
                    self.update_onion_for_last_pear_event_processed(&event, onion.onion_id)
                        .await?;
                }
                Err(e) => {
                    // Some log
                    self.block_onion(onion).await?;
                    return Err(e);
                }
            }
        }

        Ok(event_count as u64)
    }

    async fn process_new_pear_event_with_retries(
        &mut self,
        event: &PearEvent,
        onion: Onion,
    ) -> Result<()> {
        // Some retry logic based on backoffs
        Ok(())
    }

    async fn update_onion_for_last_pear_event_processed(
        &mut self,
        event: &PearEvent,
        onion_id: Uuid,
    ) -> Result<()> {
        let updates = OnionOptionalUpdates {
            // some logic of how we build OptionalUpdates
            ..Default::default()
        };

        UpdateOnionUseCase {
            onion_repository: self.onion_repo,
        }
        .execute(onion_id, updates, ExpectedOldValues::default())
        .await
    }

    async fn block_onion(&mut self, onion: &Onion) -> Result<()> {
        let updates = OnionOptionalUpdates {
            // some logic of how we build OptionalUpdates
            ..Default::default()
        };

        let expected_values = ExpectedOldValues {
            // If new Apple started, we don't want to block its processing
            apple_id: Some(onion.apple_id.clone()),
        };

        UpdateOnionUseCase {
            onion_repository: self.onion_repo,
        }
        .execute(onion.onion_id, updates, expected_values)
        .await
    }
}
