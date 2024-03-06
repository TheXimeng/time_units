use std::time::Duration;

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
