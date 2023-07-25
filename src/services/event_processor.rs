use crate::{
    entities::{apple_event::AppleEvent, onion::Onion, pear_event::PearEvent},
    use_cases::process_new_events::{AppleEventKind, ProcessAppleEvent, ProcessPearEvent},
    Result,
};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
// removed tonn of imports

#[derive(Clone)]
pub struct InternalServiecConfigNotFromThere {
    // some fields
}

pub struct EventProcessorParams {
    // Some params
    pub cfg: InternalServiecConfigNotFromThere,
}

#[derive(Clone)]
pub struct EventProcessor {
    // some fields
    cfg: InternalServiecConfigNotFromThere,
}

#[async_trait]
impl ProcessPearEvent for EventProcessor {
    async fn process_pear_event(&mut self, event: PearEvent, onion: Onion) -> Result<()> {
        // impl
        Ok(())
    }
}

#[async_trait]
impl ProcessAppleEvent for EventProcessor {
    async fn process_apple_event(&mut self, event: AppleEvent, onion: Onion) -> Result<()> {
        // impl
        Ok(())
    }

    fn get_apple_event_kind(&self, event: AppleEvent) -> Result<AppleEventKind> {
        // impl
        Ok(AppleEventKind::SomeKind)
    }
}

struct SomeTx<'a> {
    // some tx
    some_field: &'a str,
}
pub type InternalTxImplNotFromThere<'a> = Arc<Mutex<SomeTx<'a>>>;

struct ProcessParams<'a> {
    tx: InternalTxImplNotFromThere<'static>,
    onion: Onion,
    some_buffer: &'a mut Vec<u8>,
}

struct IngestParams {
    // tonn of clients and services removed
}

impl EventProcessor {
    pub async fn new(params: EventProcessorParams) -> Result<Self> {
        Ok(Self { cfg: params.cfg })
    }

    // here is interesting case with lifetimes
    // so i have left it also

    async fn process_smth<'c, 'd>(
        &'d mut self,
        event: Smth,
        process_params: &'d mut ProcessParams<'c>,
    ) -> Result<()>
    where
        'c: 'd,
    {
        self.process_smth_item(event.kind, process_params, None)
            .await
    }

    async fn process_smth_item<'c, 'd>(
        &'d mut self,
        item: SmthItem,
        process_params: &'d mut ProcessParams<'c>,
        some_id: Option<&str>,
    ) -> Result<()>
    where
        'c: 'd,
    {
        let tx = Arc::clone(&process_params.tx);

        let mut ingest_params = self.build_ingest_params().await?;
        let mut ingest_context = self
            .build_ingest_context(process_params, &mut ingest_params)
            .await?;

        // some processing with context

        Ok(())
    }

    async fn build_ingest_params(&mut self) -> Result<IngestParams> {
        // tonn of clients intiialization
        Ok(IngestParams {})
    }

    async fn build_ingest_context<'c, 'd>(
        &'d mut self,
        process_params: &'d mut ProcessParams<'c>,
        ingest_params: &'d mut IngestParams,
    ) -> Result<IngestContextNotFromThere<'d>>
    where
        'c: 'd,
    {
        Ok(IngestContextNotFromThere {
            // some fields
            some_field: "some value",
        })
    }
}

struct IngestContextNotFromThere<'a> {
    // some fields
    some_field: &'a str,
}

struct Smth {
    kind: SmthItem,
}

struct SmthItem {}
