use crate::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// We want to update only specific field
#[derive(Default, Clone)]
pub struct OnionOptionalUpdates {
    //  Batch of fields removed
    pub apple_id: Option<Option<String>>,
}

// Some times we may want to update to replace exact value
#[derive(Default, Clone)]
pub struct ExpectedOldValues {
    pub apple_id: Option<Option<String>>,
}

#[async_trait]
pub trait UpdateOnion: Send + Sync {
    async fn update(
        &self,
        onion_id: Uuid,
        optional_updates: OnionOptionalUpdates,
        expected_old_values: ExpectedOldValues,
    ) -> Result<()>;
}

pub struct UpdateOnionUseCase<'a> {
    pub onion_repository: &'a mut dyn UpdateOnion,
}

impl UpdateOnionUseCase<'_> {
    pub async fn execute(
        &self,
        onion_id: Uuid,
        optional_updates: OnionOptionalUpdates,
        expected_old_values: ExpectedOldValues,
    ) -> Result<()> {
        self.onion_repository
            .update(onion_id, optional_updates, expected_old_values)
            .await
    }
}
