//! Copiner's framework-independent domain kernel.
//!
//! This crate defines workflow and safety boundaries only. Provider SDKs, UI,
//! persistence, network access, and enterprise connectors belong to adapters.

pub mod harness;
pub mod policy;
pub mod provider;
pub mod tool;
pub mod workflow;

pub use workflow::{Workflow, WorkflowError, WorkflowEvent, WorkflowState};
