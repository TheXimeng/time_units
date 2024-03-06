//! Boilerplate code for creating [`std::time::Duration`]'s.\
//! Extends: [`u64`] - [`u8`], [`i64`] - [`i8`] and [`f64`], [`f32`]\
//! with the following methods:
//! * Seconds - [`AsDuration::secs`]
//! * Milliseconds - [`AsDuration::millis`]
//! * Microseconds - [`AsDuration::micros`]
//! * Nanoseconds - [`AsDuration::nanos`]
//!
//! # Examples
//! using [`std::thread::sleep`]
//!
//! ```no_run
//! use duration_ext::*;
//! use std::thread::sleep;
//!
//! fn main() {
//!     sleep(0.5.secs());
//!     sleep(500.millis());
//!     sleep(400_000.micros() + 100_000_000.nanos());
//!     sleep(2.secs() - 1500.millis());
//! }
//! ```

use std::time::Duration;

/// works as extension on types to support translation into [`std::time::Duration`]
///
pub trait AsDuration {
    fn secs(&self) -> Duration;
    fn millis(&self) -> Duration;
    fn micros(&self) -> Duration;
    fn nanos(&self) -> Duration;
}

macro_rules! implAsDurationUInt {
    ($ty:ident) => {
        impl AsDuration for $ty {
            fn secs(&self) -> Duration {
                Duration::from_secs(*self as u64)
            }

            fn millis(&self) -> Duration {
                Duration::from_millis(*self as u64)
            }

            fn micros(&self) -> Duration {
                Duration::from_micros(*self as u64)
            }

            fn nanos(&self) -> Duration {
                Duration::from_nanos(*self as u64)
            }
        }
    };
}

implAsDurationUInt!(u64);
implAsDurationUInt!(u32);
implAsDurationUInt!(u16);
implAsDurationUInt!(u8);

macro_rules! implAsDurationInt {
    ($ty:ident) => {
        impl AsDuration for $ty {
            fn secs(&self) -> Duration {
                debug_assert!(*self >= 0);
                Duration::from_secs(*self as u64)
            }

            fn millis(&self) -> Duration {
                debug_assert!(*self >= 0);
                Duration::from_millis(*self as u64)
            }

            fn micros(&self) -> Duration {
                debug_assert!(*self >= 0);
                Duration::from_micros(*self as u64)
            }

            fn nanos(&self) -> Duration {
                debug_assert!(*self >= 0);
                Duration::from_nanos(*self as u64)
            }
        }
    };
}

implAsDurationInt!(i64);
implAsDurationInt!(i32);
implAsDurationInt!(i16);
implAsDurationInt!(i8);

macro_rules! implAsDuratinoFloat {
    ($ty:ident) => {
        impl AsDuration for $ty {
            fn secs(&self) -> Duration {
                Duration::from_secs_f64(*self as f64)
            }

            fn millis(&self) -> Duration {
                Duration::from_secs_f64(*self as f64 / 1_000.)
            }

            fn micros(&self) -> Duration {
                Duration::from_secs_f64(*self as f64 / 1_000_000.)
            }

            fn nanos(&self) -> Duration {
                Duration::from_secs_f64(*self as f64 / 1_000_000_000.)
            }
        }
    };
}

implAsDuratinoFloat!(f64);
implAsDuratinoFloat!(f32);
