//! Keymap-related request handlers

use core::cell::RefCell;
use rmk_protocol::{
    endpoints::keymap::*,
    types::{EmptyResponse, Action}, 
    RmkError,
};
use crate::{
    keymap::KeyMap,
    event::KeyboardEventPos,
    via::keycode_convert::{from_via_keycode, to_via_keycode},
};

/// Handle get keycode request
pub async fn handle_get_keycode<
    const ROW: usize,
    const COL: usize,
    const NUM_LAYER: usize,
    const NUM_ENCODER: usize,
>(
    req: GetKeycodeRequest,
    keymap: &RefCell<KeyMap<'_, ROW, COL, NUM_LAYER, NUM_ENCODER>>,
) -> Result<GetKeycodeResponse, RmkError> {
    let pos = req.position;
    
    // Validate position
    let (row_num, col_num, layer_num) = keymap.borrow().get_keymap_config();
    if pos.row >= row_num as u8 || pos.col >= col_num as u8 || pos.layer >= layer_num as u8 {
        return Err(RmkError::InvalidKeyPosition {
            layer: pos.layer,
            row: pos.row,
            col: pos.col,
        });
    }

    // Get the keycode
    let action = keymap
        .borrow()
        .get_action_at(KeyboardEventPos::key_pos(pos.col, pos.row), pos.layer as usize);
    
    // TODO: Convert from internal action to protocol action
    let protocol_action = Action::No; // Placeholder conversion
    
    Ok(GetKeycodeResponse { action: protocol_action })
}

/// Handle set keycode request
pub async fn handle_set_keycode<
    const ROW: usize,
    const COL: usize,
    const NUM_LAYER: usize,
    const NUM_ENCODER: usize,
>(
    req: SetKeycodeRequest,
    keymap: &RefCell<KeyMap<'_, ROW, COL, NUM_LAYER, NUM_ENCODER>>,
) -> Result<EmptyResponse, RmkError> {
    let pos = req.position;
    
    // Validate position
    let (row_num, col_num, layer_num) = keymap.borrow().get_keymap_config();
    if pos.row >= row_num as u8 || pos.col >= col_num as u8 || pos.layer >= layer_num as u8 {
        return Err(RmkError::InvalidKeyPosition {
            layer: pos.layer,
            row: pos.row,
            col: pos.col,
        });
    }

    // Convert protocol action to internal action and set it
    // TODO: Implement conversion from protocol action to internal action
    let internal_action = crate::action::KeyAction::No; // Placeholder conversion
    keymap.borrow_mut().set_action_at(
        KeyboardEventPos::key_pos(pos.col, pos.row),
        pos.layer as usize,
        internal_action,
    );

    // TODO: Save to storage
    #[cfg(feature = "storage")]
    {
        use crate::{channel::FLASH_CHANNEL, storage::FlashOperationMessage};
        let _ = FLASH_CHANNEL.try_send(FlashOperationMessage::KeymapKey {
            layer: pos.layer,
            col: pos.col,
            row: pos.row,
            action: internal_action,
        });
    }

    Ok(EmptyResponse)
}