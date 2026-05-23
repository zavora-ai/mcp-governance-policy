//! # Governance Policy MCP Server
//!
//! Policy evaluation, approvals, simulation, and audit evidence for ADK-Rust Enterprise.
//! The gatekeeper that should not be bypassable by downstream MCPs.

pub mod types;
pub mod store;
pub mod server;

pub use types::*;
pub use store::PolicyStore;
