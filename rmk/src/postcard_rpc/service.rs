//! Main postcard-rpc service implementation

use crate::RawMutex;
use postcard_rpc::{define_dispatch, server::SpawnContext};
use rmk_protocol::{
    endpoints::RMK_ENDPOINT_LIST,
    topics::{RMK_TOPICS_IN_LIST, RMK_TOPICS_OUT_LIST},
};

// Import the postcard-rpc embassy-usb implementation
use postcard_rpc::server::impls::embassy_usb_v0_5::dispatch_impl::{WireSpawnImpl, WireTxImpl, spawn_fn};

// Import the handler functions
use crate::postcard_rpc::handlers::system::handle_get_protocol_version;

/// Application context for RMK postcard-rpc service
/// This struct must be 'static to satisfy postcard-rpc requirements
pub struct RmkContext {
    // Context can be extended later if needed
}

impl RmkContext {
    /// Create a new RMK application context
    pub fn new() -> Self {
        Self {}
    }
}

impl SpawnContext for RmkContext {
    type SpawnCtxt = RmkTaskContext;

    fn spawn_ctxt(&mut self) -> Self::SpawnCtxt {
        RmkTaskContext {}
    }
}

/// Task context for spawned RMK handlers
pub struct RmkTaskContext {
    // Task context can be extended later if needed
}

define_dispatch! {
    app: RmkApp;
    spawn_fn: spawn_fn;
    tx_impl: WireTxImpl;
    spawn_impl: WireSpawnImpl;
    context: RmkContext;

    endpoints: {
        list: RMK_ENDPOINT_LIST;

        | EndpointTy         | kind     | handler                        |
        | ---------------    | ----     | -------                        |
        | GetProtocolVersion | async    | handle_get_protocol_version   |
    };

    topics_in: {
        list: RMK_TOPICS_IN_LIST;

        | TopicTy                   | kind      | handler                       |
        | ----------                | ----      | -------                       |
    };

    topics_out: {
        list: RMK_TOPICS_OUT_LIST;
    };
}
