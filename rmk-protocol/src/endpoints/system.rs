//! System related endpoints
//!
//! This module defines endpoints for system operations including
//! version queries, bootloader jump, storage reset, and debugging.

use crate::types::{EmptyRequest, EmptyResponse, KeyboardInfo, MatrixState, Version};
use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

/// Response containing protocol version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct VersionResponse {
    /// Protocol version
    pub version: Version,
}

/// Response containing keyboard information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct KeyboardInfoResponse {
    /// Keyboard information
    pub info: KeyboardInfo,
}

/// Response containing matrix state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct MatrixStateResponse {
    /// Current matrix state
    pub state: MatrixState,
}

// Note: Endpoint marker types are defined in the endpoints! macro in mod.rs
// endpoint!(GetProtocolVersion, EmptyRequest, VersionResponse);
// endpoint!(GetKeyboardInfo, EmptyRequest, KeyboardInfoResponse);
// endpoint!(JumpBootloader, EmptyRequest, EmptyResponse);
// endpoint!(ResetStorage, EmptyRequest, EmptyResponse);

// #[cfg(feature = "matrix_tester")]
// endpoint!(GetMatrixState, EmptyRequest, MatrixStateResponse);
