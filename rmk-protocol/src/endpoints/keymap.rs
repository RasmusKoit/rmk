//! Keymap related endpoints
//!
//! This module defines endpoints for keyboard mapping operations including
//! getting/setting keycodes, layer management, and bulk operations.

use crate::types::{Action, EmptyRequest, EmptyResponse, KeyPosition};
use postcard_rpc::endpoint;
use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

/// Request to get a keycode at specific position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct GetKeycodeRequest {
    /// Position of the key
    pub position: KeyPosition,
}

/// Response containing a key action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct GetKeycodeResponse {
    /// The action at the requested position
    pub action: Action,
}

/// Request to set a key action at specific position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct SetKeycodeRequest {
    /// Position of the key
    pub position: KeyPosition,
    /// New action to set
    pub action: Action,
}

/// Response containing layer count
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct LayerCountResponse {
    /// Number of layers
    pub count: u8,
}

/// Request for bulk keymap buffer operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct BufferRequest {
    /// Starting offset in the keymap buffer
    pub offset: u16,
    /// Number of actions to read
    pub size: u16,
}

/// Response containing keymap buffer data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct BufferResponse {
    /// Buffer data (actions) - fixed size array
    pub data: [Action; 32],
    /// Actual number of valid actions in the buffer
    pub count: u16,
}

/// Request to update keymap buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct BufferUpdateRequest {
    /// Starting offset in the keymap buffer
    pub offset: u16,
    /// Actions to write - fixed size array
    pub data: [Action; 32],
    /// Actual number of actions to write
    pub count: u16,
}

// Define endpoints for keymap operations
endpoint!(GetKeycode, GetKeycodeRequest, GetKeycodeResponse);
endpoint!(SetKeycode, SetKeycodeRequest, EmptyResponse);
endpoint!(GetLayerCount, EmptyRequest, LayerCountResponse);
endpoint!(GetKeymapBuffer, BufferRequest, BufferResponse);
endpoint!(SetKeymapBuffer, BufferUpdateRequest, EmptyResponse);
