//! RMK Protocol definitions for postcard-rpc communication
//! 
//! This crate defines all the endpoints and data types used for communication
//! between RMK firmware and host PC using postcard-rpc.

#![no_std]
#![warn(missing_docs)]

/// Re-export postcard-rpc for convenience
pub use postcard_rpc;

/// Endpoint definitions
pub mod endpoints;

/// Topic definitions  
pub mod topics;

/// Data type definitions
pub mod types;

/// Error definitions
pub mod errors;

// Re-export commonly used items
pub use errors::RmkError;
pub use postcard_rpc::{endpoint, Endpoint, Topic};
