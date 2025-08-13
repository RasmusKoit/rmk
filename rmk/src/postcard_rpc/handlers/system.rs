//! System-related request handlers

use core::cell::RefCell;
use postcard_rpc::header::VarHeader;
use rmk_protocol::{
    endpoints::system::*,
    types::{EmptyRequest, EmptyResponse, KeyboardInfo, Version, PROTOCOL_VERSION},
    RmkError,
};
use crate::{
    keymap::KeyMap,
    boot,
};

/// Handle get protocol version request
pub async fn handle_get_protocol_version(
    _context: &mut crate::postcard_rpc::service::RmkContext,
    _header: VarHeader,
    _req: EmptyRequest,
) -> VersionResponse {
    VersionResponse {
        version: PROTOCOL_VERSION,
    }
}

/// Handle get keyboard info request
pub async fn handle_get_keyboard_info(
    _context: &mut crate::postcard_rpc::service::RmkContext,
    _header: VarHeader,
    _req: EmptyRequest,
) -> KeyboardInfoResponse {
    // TODO: Get actual keyboard dimensions from configuration
    let info = KeyboardInfo {
        protocol_version: PROTOCOL_VERSION,
        firmware_version: Version { major: 0, minor: 7, patch: 8 },
        rows: 0, // Placeholder
        cols: 0, // Placeholder  
        layers: 0, // Placeholder
        encoders: 0, // Placeholder
        locked: false, // TODO: Get actual lock status
    };
    
    KeyboardInfoResponse { info }
}

/// Handle jump to bootloader request
pub async fn handle_jump_bootloader(
    _context: &mut crate::postcard_rpc::service::RmkContext,
    _header: VarHeader,
    _req: EmptyRequest,
) -> EmptyResponse {
    warn!("Jumping to bootloader");
    boot::jump_to_bootloader();
    // This should never return
    EmptyResponse
}

/// Handle reset storage request
pub async fn handle_reset_storage(
    _context: &mut crate::postcard_rpc::service::RmkContext,
    _header: VarHeader,
    _req: EmptyRequest,
) -> EmptyResponse {
    warn!("Resetting storage");
    #[cfg(feature = "storage")]
    {
        use crate::{channel::FLASH_CHANNEL, storage::FlashOperationMessage};
        FLASH_CHANNEL.send(FlashOperationMessage::Reset).await;
    }
    EmptyResponse
}