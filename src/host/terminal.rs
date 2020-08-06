use crate::prelude::*;

use std::sync::atomic::{
    AtomicU8,
    Ordering,
};

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Flag {
    UsesAlternateScreen = 1 << 0,
    MouseCaptured = 1 << 1,
    RawModeEnabled = 1 << 2,
}

pub type Flags = BitFlags<Flag>;

static CURRENT_FLAGS: AtomicU8 = AtomicU8::new(0);

#[inline]
pub fn current_flags() -> Result<Flags> {
    let bits = CURRENT_FLAGS.load(Ordering::SeqCst);
    Flags::try_from(bits).map_err(|e| anyhow!("Error, {0}, {0:?}", e))
}

#[inline]
pub fn set_current_flags(flags: Flags) {
    CURRENT_FLAGS.store(flags.bits(), Ordering::SeqCst)
}
