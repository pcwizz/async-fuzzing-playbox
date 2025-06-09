use std::marker::PhantomData;
use typed_builder::TypedBuilder;

use libafl::{
    Error,
    events::{ClientDescription, SimpleEventManager},
    inputs::BytesInput,
    monitors::{Monitor, stats::ClientStatsManager},
};

use libafl_bolts::ClientId;

use crate::client::ClientState;

pub type ClientMgr<M> = SimpleEventManager<BytesInput, M, ClientState>;

#[derive(TypedBuilder)]
pub struct Instance<M: Monitor> {
    mgr: ClientMgr<M>,
    client_description: ClientDescription,
    #[builder(default)]
    extra_tokens: Vec<String>,
    #[builder(default=PhantomData)]
    phantom: PhantomData<M>,
}

impl<M: Monitor> Instance<M> {
    pub fn display(
        &mut self,
        client_stats_manager: &mut ClientStatsManager,
        event_msg: &str,
        sender_id: ClientId,
    ) -> Result<(), Error> {
        Ok(())
    }
}
