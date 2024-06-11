//! Crate defining a program for use with the transfer hook extension
//! that checks wheather the NYSE is open for trading.
//! If not open the program will return an error, thus disallowing
//! the transfer.

#![allow(clippy::arithmetic_side_effects)]
#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

pub mod errors;
pub mod processor;
pub mod utils;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk
// version
pub use solana_program;
