use crate::{entities::onion::Onion, Result};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait GetOnion: Send + Sync {
    async fn get_onion(&self, some_id: Uuid) -> Result<Option<Onion>>;
}

pub struct GetOnionUseCase<'a> {
    pub onion_repo: &'a mut dyn GetOnion,
}

impl GetOnionUseCase<'_> {
    pub async fn execute(&self, some_id: Uuid) -> Result<Option<Onion>> {
        self.onion_repo.get_onion(some_id).await
    }
}
