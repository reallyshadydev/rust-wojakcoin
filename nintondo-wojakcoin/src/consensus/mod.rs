// SPDX-License-Identifier: CC0-1.0

//! pepecoin consensus.
//!
//! This module defines structures, functions, and traits that are needed to
//! conform to pepecoin consensus.
//!

pub mod encode;
pub mod params;

pub use self::encode::{deserialize, deserialize_partial, serialize};
pub use self::encode::{Decodable, Encodable, ReadExt, WriteExt};
pub use self::params::Params;

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;
