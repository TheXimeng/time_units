# Time Units

Boilerplate code for creating [`std::time::Duration`]'s.\
Extends: [`u64`] - [`u8`], [`i64`] - [`i8`] and [`f64`], [`f32`]\
with the following methods:
* Seconds - [`AsDuration::secs`]
* Milliseconds - [`AsDuration::millis`]
* Microseconds - [`AsDuration::micros`]
* Nanoseconds - [`AsDuration::nanos`]

# Usage
in **Cargo.toml**:
```toml
[dependencies]
time_units = { git = "https://github.com/TheXimeng/time_units.git" }
```

## Examples
using [`std::thread::sleep`]

```rust
use duration_ext::*;
use std::thread::sleep;

fn main() {
    sleep(0.5.secs());
    sleep(500.millis());
    sleep(400_000.micros() + 100_000_000.nanos());
    sleep(2.secs() - 1500.millis());
}
```
