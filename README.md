# hhmmss

Format time and duration in chrono, std::time and time in the formats `HH:MM:SS`,  `HH:MM:SS.xxx` milli seconds, `HH:MM:SS.xxxxxx` micro seconds or  `HH:MM:SS.xxxxxxxxx` nano seconds.

## Why?

I just wanted to format `chrono::Duration` as `HH:MM:SS` in some of my crates. However there isn't a ready-to-use method for this. Format is only supported for instances. Neither `time` nor `std::time`'s `Duration` support this. So I made this crate, mainly for my own convinience.

# Usage

Add `hhmmss` to `Cargo.toml`:

```toml
[dependencies]
hhmmss = "0.1"
```

Then:

```rust
use hhmmss::Hhmmss;

fn main() {
    let std_duration = std::time::Duration::new(3661, 987_654_321);
    assert_eq!(&std_duration.hhmmss(), "01:01:01");
    assert_eq!(&std_duration.hhmmssxxx(), "01:01:01.987");
    assert_eq!(&std_duration.hhmmssxxxxxx(), "01:01:01.987654");
    assert_eq!(&std_duration.hhmmssxxxxxxxxx(), "01:01:01.987654321");
    
    let chrono_duration = chrono::Duration::from_std(std_duration).unwrap(); // needs chrono
    assert_eq!(&chrono_duration.hhmmss(), "01:01:01");
    assert_eq!(&chrono_duration.hhmmssxxx(), "01:01:01.987");
    assert_eq!(&chrono_duration.hhmmssxxxxxx(), "01:01:01.987654");
    assert_eq!(&chrono_duration.hhmmssxxxxxxxxx(), "01:01:01.987654321");
    
    let time_duration = time::Duration::from_std(std_duration).unwrap(); // needs time
    assert_eq!(&time_duration.hhmmss(), "01:01:01");
    assert_eq!(&time_duration.hhmmssxxx(), "01:01:01.987");
    assert_eq!(&time_duration.hhmmssxxxxxx(), "01:01:01.987654");
    assert_eq!(&time_duration.hhmmssxxxxxxxxx(), "01:01:01.987654321");
}
```