use super::*;
use crate::prelude::*;
use std::sync::atomic::{
    AtomicBool, AtomicU32, AtomicUsize, Ordering::Relaxed,
};

/// A convenience trait for shorthand `Relaxed` atomic load and store operations.
pub trait AtomicOps {
    /// The non-atomic type for this type.
    type NonAtomic;

    /// Convenience method for `Relaxed` atomic loading.
    fn lr(&self) -> Self::NonAtomic;

    /// Convenience method for `Relaxed` atomic storing.
    fn sr(&self, val: Self::NonAtomic);
}

impl AtomicOps for Atomic<f32> {
    type NonAtomic = f32;

    fn lr(&self) -> Self::NonAtomic {
        self.load(Relaxed)
    }

    fn sr(&self, val: Self::NonAtomic) {
        self.store(val, Relaxed);
    }
}

impl AtomicOps for Atomic<f64> {
    type NonAtomic = f64;

    fn lr(&self) -> Self::NonAtomic {
        self.load(Relaxed)
    }

    fn sr(&self, val: Self::NonAtomic) {
        self.store(val, Relaxed);
    }
}

impl AtomicOps for AtomicU32 {
    type NonAtomic = u32;

    fn lr(&self) -> Self::NonAtomic {
        self.load(Relaxed)
    }

    fn sr(&self, val: Self::NonAtomic) {
        self.store(val, Relaxed);
    }
}

impl AtomicOps for AtomicUsize {
    type NonAtomic = usize;

    fn lr(&self) -> Self::NonAtomic {
        self.load(Relaxed)
    }

    fn sr(&self, val: Self::NonAtomic) {
        self.store(val, Relaxed);
    }
}

impl AtomicOps for AtomicBool {
    type NonAtomic = bool;

    fn lr(&self) -> Self::NonAtomic {
        self.load(Relaxed)
    }

    fn sr(&self, val: Self::NonAtomic) {
        self.store(val, Relaxed);
    }
}
