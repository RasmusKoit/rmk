//! Postcard-RPC service module for RMK
//!
//! This module provides communication between RMK firmware and host PC
//! using the postcard-rpc protocol over USB serial.

pub mod service;

pub mod transport;

pub mod handlers;

// pub use service::{RmkApp, RmkContext};
