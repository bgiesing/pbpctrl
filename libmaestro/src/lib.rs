//! Library for the Maestro protocol used to change settings (ANC, equalizer,
//! etc.) on the Google Pixel Buds Pro. Might support other Pixel Buds, might
//! not.

use uuid::{uuid, Uuid};

/// UUID under which the Maestro protocol is advertised.
///
/// Defined as `25e97ff7-24ce-4c4c-8951-f764a708f7b5`.
pub const UUID: Uuid = uuid!("3a046f6d-24d2-7655-6534-0d7ecb759709");

pub mod hdlc;
pub mod protocol;
pub mod pwrpc;
pub mod service;
