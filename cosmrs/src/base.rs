//! Base functionality.

mod account_id;
mod coin;
mod denom;
mod any;

/// Query support
pub mod query;

pub use self::{account_id::AccountId, coin::Coin, denom::Denom, any::Any};

/// Amounts.
pub type Amount = u128;

/// Gas cost.
pub type Gas = u64;
