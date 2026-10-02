//! Module declarations and re-exports

/// Authentication types and scope definitions.
pub mod auth;

/// Common foundational types, unit enums, and error models.
pub mod common;

/// Credit balance and usage quota models.
pub mod credits;

/// Food analysis models for image, text, and correction endpoints.
pub mod food_analysis;

/// Food search, autocomplete, details, and alternatives models.
pub mod foods;

/// Glucose prediction models.
pub mod glucose;

/// Logging models for food, water, and weight entries.
pub mod logs;

/// Restaurant search and menu item lookup models.
pub mod restaurants;

pub use auth::*;
pub use common::*;
pub use credits::*;
pub use food_analysis::*;
pub use foods::*;
pub use glucose::*;
pub use logs::*;
pub use restaurants::*;
