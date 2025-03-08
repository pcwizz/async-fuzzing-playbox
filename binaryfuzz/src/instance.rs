use std::fmt::Debug;
use std::marker::PhantomData;
use typed_builder::TypedBuilder;

use libafl::{
    Error,
    events::{ClientDescription, SimpleEventManager},
    inputs::BytesInput,
    monitors::Monitor,
};

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

use libafl_qemu::modules::EmulatorModuleTuple;

impl<M: Monitor> Instance<M> {
    pub fn run<ET>(
        &mut self,
        args: Vec<String>,
        modules: ET,
        state: Option<ClientState>,
    ) -> Result<(), Error>
    where
        ET: EmulatorModuleTuple<BytesInput, ClientState> + Debug,
    {
        Ok(())
    }
}
