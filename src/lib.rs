#![forbid(unsafe_code)]

#[cfg(not(feature = "no_entrypoint"))]
mod entrypoint;

pub mod processor;
pub mod instruction;
pub mod state;

solana_program::declare_id!("invoker333333333333333333333333333333333399");