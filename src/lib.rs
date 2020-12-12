pub trait Hhmmss {
    fn sms(&self) -> (i64, i64);
    /// Pretty-prints a chrono::Duration in the form `HH:MM:SS.xxx`
    fn hhmmss(&self) -> String {
        let (s, _ms) = self.sms();
        s2hhmmss(s)
    }
    /// Pretty-prints a chrono::Duration in the form `HH:MM:SS.xxx` using milli second resolution
    fn hhmmssxxx(&self) -> String {
        let (s, ns) = self.sms();
        sms2hhmmssxxx(s, ns / 1_000_000)
    }
    /// Pretty-prints a chrono::Duration in the form `HH:MM:SS.xxxxxx` using micro second resolution
    fn hhmmssxxxxxx(&self) -> String {
        let (s, ns) = self.sms();
        sms2hhmmssxxxxxx(s, ns / 1_000)
    }

    /// Pretty-prints a chrono::Duration in the form `HH:MM:SS.xxxxxxxxx` using nano second resolution
    fn hhmmssxxxxxxxxx(&self) -> String {
        let (s, ns) = self.sms();
        sms2hhmmssxxxxxxxxx(s, ns)
    }
}

impl Hhmmss for chrono::Duration {
    fn sms(&self) -> (i64, i64) {
        let s = self.num_seconds();
        let mut res_ns = self.num_milliseconds() * 1_000_000 - 1_000_000_000 * s;
        if let Some(um) = self.num_microseconds() {
            res_ns = um * 1_000 - 1_000_000 * s;
        }
        if let Some(ns) = self.num_nanoseconds() {
            res_ns = ns - 1_000_000_000 * s;
        }
        (s, res_ns)
    }
}

impl Hhmmss for std::time::Duration {
    fn sms(&self) -> (i64, i64) {
        let s = self.as_secs();
        let ns = self.subsec_nanos();
        (s as i64, ns as i64)
    }
}

impl Hhmmss for time::Duration {
    fn sms(&self) -> (i64, i64) {
        let s = self.whole_seconds();
        let ns = self.whole_nanoseconds() as i64 - 1_000_000_000 * s;
        (s, ns)
    }
}

fn s2hhmmss(s: i64) -> String {
    let mut neg = false;
    let mut s = s;
    if s < 0 {
        neg = true;
        s = -s;
    }
    let (h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    format!("{}{:02}:{:02}:{:02}", if neg { "-" } else { "" }, h, m, s)
}

fn sms2hhmmssxxx(s: i64, ms: i64) -> String {
    let mut neg = false;
    let (mut s, mut ms) = (s, ms);
    if s < 0 {
        neg = true;
        s = -s;
        ms = -ms;
    }
    let (h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    format!(
        "{}{:02}:{:02}:{:02}.{:03}",
        if neg { "-" } else { "" },
        h,
        m,
        s,
        ms
    )
}

fn sms2hhmmssxxxxxx(s: i64, mu: i64) -> String {
    let mut neg = false;
    let (mut s, mut mu) = (s, mu);
    if s < 0 {
        neg = true;
        s = -s;
        mu = -mu;
    }
    let (h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    format!(
        "{}{:02}:{:02}:{:02}.{:06}",
        if neg { "-" } else { "" },
        h,
        m,
        s,
        mu
    )
}

fn sms2hhmmssxxxxxxxxx(s: i64, ns: i64) -> String {
    let mut neg = false;
    let (mut s, mut ns) = (s, ns);
    if s < 0 {
        neg = true;
        s = -s;
        ns = -ns;
    }
    let (h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    format!(
        "{}{:02}:{:02}:{:02}.{:09}",
        if neg { "-" } else { "" },
        h,
        m,
        s,
        ns
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let std_duration = std::time::Duration::new(3661, 987_654_321);
        assert_eq!(&std_duration.hhmmss(), "01:01:01");
        assert_eq!(&std_duration.hhmmssxxx(), "01:01:01.987");
        assert_eq!(&std_duration.hhmmssxxxxxx(), "01:01:01.987654");
        assert_eq!(&std_duration.hhmmssxxxxxxxxx(), "01:01:01.987654321");
        let chrono_duration = chrono::Duration::from_std(std_duration).unwrap();
        assert_eq!(&chrono_duration.hhmmss(), "01:01:01");
        assert_eq!(&chrono_duration.hhmmssxxx(), "01:01:01.987");
        assert_eq!(&chrono_duration.hhmmssxxxxxx(), "01:01:01.987654");
        assert_eq!(&chrono_duration.hhmmssxxxxxxxxx(), "01:01:01.987654321");
        let time_duration = time::Duration::from_std(std_duration).unwrap();
        assert_eq!(&time_duration.hhmmss(), "01:01:01");
        assert_eq!(&time_duration.hhmmssxxx(), "01:01:01.987");
        assert_eq!(&time_duration.hhmmssxxxxxx(), "01:01:01.987654");
        assert_eq!(&time_duration.hhmmssxxxxxxxxx(), "01:01:01.987654321");
    }
}
