//! Endpoint definitions
//! 
//! This module contains all endpoint definitions organized by functionality.

pub mod keymap;
pub mod system;

// Re-export all endpoints for convenience
pub use keymap::*;
pub use system::*;

use postcard_rpc::endpoints;
use crate::types::EmptyRequest;

// Define endpoint lists for the dispatch system
endpoints! {
    list = RMK_ENDPOINT_LIST;
    | EndpointTy         | RequestTy    | ResponseTy       | Path                    |
    | ---------------    | ---------    | ----------       | ----                    |
    | GetProtocolVersion | EmptyRequest | VersionResponse  | "rmk/system/version"    |
}