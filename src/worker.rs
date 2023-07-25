use crate::{
    use_cases::{
        get_onion::GetOnion,
        process_new_events::{
            ListSequentialPearEvents, ListSequentialAppleEvents,
            ProcessNewEvents, ProcessPearEvent, ProcessAppleEvent,
        },
        update_onion::UpdateOnion,
    },
};
use async_trait::async_trait;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
// removed tonn of imports

pub struct Worker<CCISR, RER, REP, RRER, RREP>
where
    CCISR: GetOnion + UpdateOnion + Clone + 'static,
    RER: ListSequentialPearEvents + Clone + 'static,
    REP: ProcessPearEvent + Clone + 'static,
    RRER: ListSequentialAppleEvents + Clone + 'static,
    RREP: ProcessApplePearEvent + Clone + 'static,
{
    pub onion_repo: CCISR,
    pub apple_event_repo: RRER,
    pub apple_events_processor: RREP,
    pub pear_event_repo: RER,
    pub pear_events_processor: REP,
    // some fields removed
}

// Some internal worker trait
#[async_trait]
trait SomeInternalTrait {
    async fn run(self) -> Result<(), Self::Error>;
}

#[async_trait]
impl<CCISR, RER, REP, RRER, RREP> SomeInternalTrait
    for Worker<CCISR, RER, REP, RRER, RREP>
where
    CCISR: GetOnion + UpdateOnion + Clone + 'static,
    RRER: ListSequentialAppleEvents + Clone + 'static,
    RER: ListSequentialPearEvents + Clone + 'static,
    REP: ProcessPearEvent + Clone + 'static,
    RREP: ProcessAppleEvent + Clone + 'static,
{
    async fn run(self) -> Result<(), Self::Error> {
        loop {
            // processing messages in the loop
        }
    }
}

// removed all logic of message processing
