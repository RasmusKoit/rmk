//! Error types for RMK protocol
//!
//! These errors can be returned by RMK firmware when processing requests.

use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

/// Main error type for RMK protocol operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub enum RmkError {
    /// Invalid key position specified
    InvalidKeyPosition {
        /// Layer number
        layer: u8,
        /// Row number
        row: u8,
        /// Column number
        col: u8,
    },

    /// Invalid keycode value
    InvalidKeycode(u16),

    /// Layer index out of range
    LayerOutOfRange {
        /// Maximum valid layer
        max: u8,
        /// Requested layer
        requested: u8,
    },

    /// Storage operation failed
    StorageError,

    /// Configuration is corrupted
    ConfigCorrupted,

    /// Not enough space for operation
    InsufficientSpace {
        /// Required space in bytes
        required: u32,
        /// Available space in bytes
        available: u32,
    },

    /// Macro definition too long
    MacroTooLong {
        /// Maximum allowed length
        max_len: u32,
        /// Actual length
        actual_len: u32,
    },

    /// Combo conflicts with existing combo
    ComboConflict {
        /// Index of conflicting combo
        existing_combo: u8,
    },

    /// Tap dance is in use and cannot be modified
    TapDanceInUse {
        /// Index of tap dance
        index: u8,
    },

    /// System is busy processing another request
    SystemBusy,

    /// Operation requires unlock but keyboard is locked
    Unauthorized,

    /// Hardware error occurred
    HardwareError,

    /// Protocol version mismatch
    ProtocolVersionMismatch {
        /// Expected version
        expected: u16,
        /// Actual version
        actual: u16,
    },

    /// Invalid endpoint
    InvalidEndpoint,

    /// Transport layer error
    TransportError,

    /// Feature not implemented
    NotImplemented,
}

#[cfg(not(feature = "defmt"))]
impl core::fmt::Display for RmkError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RmkError::InvalidKeyPosition { layer, row, col } => {
                write!(f, "Invalid key position: layer={}, row={}, col={}", layer, row, col)
            }
            RmkError::InvalidKeycode(code) => write!(f, "Invalid keycode: 0x{:04X}", code),
            RmkError::LayerOutOfRange { max, requested } => {
                write!(f, "Layer {} out of range (max: {})", requested, max)
            }
            RmkError::StorageError => write!(f, "Storage operation failed"),
            RmkError::ConfigCorrupted => write!(f, "Configuration is corrupted"),
            RmkError::InsufficientSpace { required, available } => {
                write!(f, "Insufficient space: required={}, available={}", required, available)
            }
            RmkError::MacroTooLong { max_len, actual_len } => {
                write!(f, "Macro too long: max={}, actual={}", max_len, actual_len)
            }
            RmkError::ComboConflict { existing_combo } => {
                write!(f, "Combo conflicts with existing combo {}", existing_combo)
            }
            RmkError::TapDanceInUse { index } => {
                write!(f, "Tap dance {} is in use", index)
            }
            RmkError::SystemBusy => write!(f, "System is busy"),
            RmkError::Unauthorized => write!(f, "Operation requires unlock"),
            RmkError::HardwareError => write!(f, "Hardware error occurred"),
            RmkError::ProtocolVersionMismatch { expected, actual } => {
                write!(f, "Protocol version mismatch: expected={}, actual={}", expected, actual)
            }
            RmkError::InvalidEndpoint => write!(f, "Invalid endpoint"),
            RmkError::TransportError => write!(f, "Transport layer error"),
            RmkError::NotImplemented => write!(f, "Feature not implemented"),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for RmkError {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            RmkError::InvalidKeyPosition { layer, row, col } => {
                defmt::write!(fmt, "Invalid key position: layer={}, row={}, col={}", layer, row, col)
            }
            RmkError::InvalidKeycode(code) => defmt::write!(fmt, "Invalid keycode: 0x{:04X}", code),
            RmkError::LayerOutOfRange { max, requested } => {
                defmt::write!(fmt, "Layer {} out of range (max: {})", requested, max)
            }
            RmkError::StorageError => defmt::write!(fmt, "Storage operation failed"),
            RmkError::ConfigCorrupted => defmt::write!(fmt, "Configuration is corrupted"),
            RmkError::InsufficientSpace { required, available } => {
                defmt::write!(
                    fmt,
                    "Insufficient space: required={}, available={}",
                    required,
                    available
                )
            }
            RmkError::MacroTooLong { max_len, actual_len } => {
                defmt::write!(fmt, "Macro too long: max={}, actual={}", max_len, actual_len)
            }
            RmkError::ComboConflict { existing_combo } => {
                defmt::write!(fmt, "Combo conflicts with existing combo {}", existing_combo)
            }
            RmkError::TapDanceInUse { index } => {
                defmt::write!(fmt, "Tap dance {} is in use", index)
            }
            RmkError::SystemBusy => defmt::write!(fmt, "System is busy"),
            RmkError::Unauthorized => defmt::write!(fmt, "Operation requires unlock"),
            RmkError::HardwareError => defmt::write!(fmt, "Hardware error occurred"),
            RmkError::ProtocolVersionMismatch { expected, actual } => {
                defmt::write!(
                    fmt,
                    "Protocol version mismatch: expected={}, actual={}",
                    expected,
                    actual
                )
            }
            RmkError::InvalidEndpoint => defmt::write!(fmt, "Invalid endpoint"),
            RmkError::TransportError => defmt::write!(fmt, "Transport layer error"),
            RmkError::NotImplemented => defmt::write!(fmt, "Feature not implemented"),
        }
    }
}
