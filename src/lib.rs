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

/// Place the mint id that you want to target with your transfer hook program.
/// Any other mint will fail to initialize, protecting the transfer hook program
/// from rogue mints trying to get access to accounts.
///
/// There are many situations where it's reasonable to support multiple mints
/// with one transfer-hook program, but because it's easy to make something
/// unsafe, this simple example implementation only allows for one mint.
#[cfg(feature = "forbid-additional-mints")]
pub mod mint {
    solana_program::declare_id!("Stockz8sdJAiizS7dcbxx4Xy1AgXZqESctEkt8t3Wu9");
}
