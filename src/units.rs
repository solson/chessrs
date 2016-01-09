//! This module provides constants for use in unit conversions. They should be multiplied with
//! the value being converted. For example:
//!
//! ```
//! use chessrs::units;
//!
//! let nanoseconds = 123456789.0;
//! let seconds = nanoseconds * units::NS_TO_S;
//! assert_eq!(seconds * units::S_TO_NS, nanoseconds);
//! ```
//!
//! Never divide by a units constant. Instead, multiply by the opposite constant (e.g.
//! `S_TO_NS` instead of `NS_TO_S`.

/// Seconds per nanosecond.
pub const NS_TO_S: f32 = 1e-9;

/// Nanoseconds per second.
pub const S_TO_NS: f32 = 1e9;
