/// This module contains the core client implementation.
pub mod client;

/// This module contains constants used by the client.
mod constants;

/// This module contains the core type definitions for the client.
pub mod types;

pub use client::{MagicedenApiConfig, MagicedenClient};
