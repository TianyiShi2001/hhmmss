# hhmmss

Format time and duration in chrono, std::time and time as `HH:MM:SS` or `HH:MM:SS.xxx`.

## Why?

I just wanted to format `chrono::Duration` as `HH:MM:SS` in some of my crates. However there isn't a ready-to-use method for this. Format is only supported for instances. Neither `time` nor `std::time`'s `Duration` support this. So I made this crate, mainly for my own convinience.

## Usage

Add `hhmmss` to `Cargo.toml`:

```toml
[dependencies]
hhmmss = "0.1"
```

Then:

```rust
use hhmmss::Hhmmss;

fn main() {
    let std_duration = std::time::Duration::new(3661, 534_000_000);
    assert_eq!(&std_duration.hhmmss(), "01:01:01");
    assert_eq!(&std_duration.hhmmssxxx(), "01:01:01.534");
    let chrono_duration = chrono::Duration::from_std(std_duration).unwrap(); // needs chrono
    assert_eq!(&chrono_duration.hhmmss(), "01:01:01"); 
    assert_eq!(&chrono_duration.hhmmssxxx(), "01:01:01.534");
    let time_duration = time::Duration::from_std(std_duration).unwrap(); // needs time
    assert_eq!(&time_duration.hhmmss(), "01:01:01");
    assert_eq!(&time_duration.hhmmssxxx(), "01:01:01.534");
}
```

## License

This project is licensed under the MIT license.
