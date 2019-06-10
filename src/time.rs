use serde::{Deserialize, Serialize};
use chrono::prelude::*;

pub const DAY: u64 = 24 * 60 * 60 * 1000;

#[derive(Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
/// A moment in time. Represented as number of milliseconds since midnight.
pub struct Moment(u64);

#[allow(dead_code)]
impl Moment {

    /// Creates a new Moment from milliseconds since midnight (clamped to 23:59:59:999).
    pub fn from_milliseconds(t: u64) -> Self {
        Self(t.min(DAY))
    }

    /// Creates a new Moment.
    pub fn new(hours: u64, minutes: u64, seconds: u64) -> Self {
        Self::from_milliseconds((hours * 60 * 60 * 1000) + (minutes * 60 * 1000) + (seconds * 1000))
    }

    pub fn new_full(hours: u64, minutes: u64, seconds: u64, milliseconds: u64) -> Self {
        Self::from_milliseconds((hours * 60 * 60 * 1000) + (minutes * 60 * 1000) + (seconds * 1000) + milliseconds)
    }

    /// Creates a new Moment. Returns None if the resulting number of milliseconds exceed a days worth.
    pub fn from(hours: u64, minutes: u64, seconds: u64) -> Option<Self> {
        let ms = (hours * 60 * 60 * 1000) + (minutes * 60 * 1000) + (seconds * 1000);
        match ms < DAY {
            true => Some(Self(ms)),
            false => None
        }
    }

    pub fn from_full(h: u64, m: u64, s: u64, ms: u64) -> Option<Self> {
        let ms = (h * 60 * 60 * 1000) + (m * 60 * 1000) + (s * 1000) + ms;
        match ms < DAY {
            true => Some(Self(ms)),
            false => None
        }
    }

    /// Time of day. Using the system clock.
    pub fn now() -> Self {
        // let epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("SystemTime is set to before EPOCH!").as_millis();
        let n = Local::now();
        let now = (n.time().num_seconds_from_midnight() * 1000) + n.timestamp_subsec_millis();

        Self::from_milliseconds((now % DAY as u32) as u64)
    }

    pub fn as_hours(&self) -> u64 {
        self.0 / 1000 / 60 / 60
    }

    pub fn as_minutes(&self) -> u64 {
        self.0 / 1000 / 60
    }

    pub fn as_seconds(&self) -> u64 {
        self.0 / 1000
    }

    pub fn as_milliseconds(&self) -> u64 {
        self.0
    }

    pub fn subsec_milliseconds(&self) -> u64 {
        self.0 % 1000
    }

    pub fn to_string(&self) -> String {
        format!("{:0>2}:{:0>2}:{:0>2}:{:0>3}", self.as_hours(), self.as_minutes() % 60, self.as_seconds() % 60, self.subsec_milliseconds())
    }

    /// Parses time from a string. Format: "h:m:s:ms", the "s:ms"-suffix is optional.
    pub fn from_string(v: &str) -> Option<Self> {
        let components: Vec<_> = v.split(":").map(|a| a.parse::<u64>()).collect();

        match (components.get(0), components.get(1), components.get(2), components.get(3)) {
            (Some(Ok(hours)), Some(Ok(minutes)), None, None) => Self::from(hours.clone(), minutes.clone(), 0),
            (Some(Ok(hours)), Some(Ok(minutes)), Some(Ok(seconds)), None) => Self::from(hours.clone(), minutes.clone(), seconds.clone()),
            (Some(Ok(hours)), Some(Ok(minutes)), Some(Ok(seconds)), Some(Ok(milliseconds))) => Some(Self::new_full(hours.clone(), minutes.clone(), seconds.clone(), milliseconds.clone())),
            _ => None
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Interval {
    pub from: Moment,
    pub to: Moment
}

#[allow(dead_code)]
impl Interval {
    /// Creates an interval from [tx, ty], where tx is the smallest/earliest in time.
    pub fn new(t1: Moment, t2: Moment) -> Self {
        if t1 < t2 {
            Self {
                from: t1,
                to: t2
            }
        } else {
            Self {
                from: t2,
                to: t1
            }
        }
    }

    pub fn contains(&self, t: Moment) -> bool {
        (self.from <= t && t < self.to)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn moment_from_hms() {
        let t0 = Moment::from(16, 30, 00);
        assert!(t0.is_some());

        let t1 = Moment::from(01, 01, 00);
        assert!(t1.is_some());

        let t2 = Moment::from(00, 00, 00);
        assert!(t2.is_some());

        let f0 = Moment::from(23, 59, 60);
        assert!(f0.is_none());

        let f1 = Moment::from(00, 86401, 00);
        assert!(f1.is_none());

        let f2 = Moment::from(24, 00, 00);
        assert!(f2.is_none());
    }

    #[test]
    fn moment_from_hms_then_to_string() {
        let t0 = Moment::new(16, 30, 00);
        assert_eq!("16:30:00:000", t0.to_string());

        let t1 = Moment::new(23, 00, 00);
        assert_eq!("23:00:00:000", t1.to_string());

        let t3 = Moment::new(01, 01, 00);
        assert_eq!("01:01:00:000", t3.to_string());
    }

    #[test]
    fn moment_from_hms_then_as_seconds() {
        let t0 = Moment::new(16, 30, 00);
        assert_eq!((16 * 60 * 60) + (30 * 60), t0.as_seconds());

        let t1 = Moment::new(23, 00, 00);
        assert_eq!((23 * 60 * 60), t1.as_seconds());

        let t3 = Moment::new(01, 01, 00);
        assert_eq!((1 * 60 * 60) + (1 * 60), t3.as_seconds());
    }

    #[test]
    fn moment_from_hms_edge_cases() {
        let t0 = Moment::from(00, 00, 00);
        let t1 = Moment::from(24, 00, 00);

        // there are 86000 seconds in a day, but 86000 is not a valid description of the time of day.
        assert_ne!(t0, t1);
    }

    #[test]
    fn moment_from_string() {
        let t0 = Moment::from_string("16:30:00");
        assert!(t0.is_some());
        assert_eq!(Moment::new(16, 30, 00), t0.unwrap());

        let t1 = Moment::from_string("23:00:00");
        assert!(t1.is_some());
        assert_eq!(Moment::new(23, 00, 00), t1.unwrap());

        let t3 = Moment::from_string("01:01:00");
        assert!(t3.is_some());
        assert_eq!(Moment::new(01, 01, 00), t3.unwrap());
    }

    #[test]
    fn moment_now() {
        let now = Moment::now();
        println!("Time now: {}", now.to_string());
    }

    #[test]
    fn moment_sorted() {
        let sorted_times = vec![
            Moment::new(00, 30, 00),
            Moment::new(01, 30, 00),
            Moment::new(16, 00, 00),
            Moment::new(23, 30, 00),
            Moment::new(23, 59, 59)
        ];

        let mut unsorted_times = vec![
            Moment::new(16, 00, 00),
            Moment::new(01, 30, 00),
            Moment::new(23, 30, 00),
            Moment::new(00, 30, 00),
            Moment::new(23, 59, 59)
        ];

        unsorted_times.sort();

        assert_eq!(sorted_times, unsorted_times);
    }

    #[test]
    fn interval_contains() {
        let i1 = Interval::new(Moment::new(16, 00, 00), Moment::new(16, 30, 00));
        assert!(i1.contains(Moment::new(16, 00, 00)));
        assert!(i1.contains(Moment::new(16, 15, 00)));
        assert!(!i1.contains(Moment::new(16, 30, 00)));

        // Check for correct ordering.
        let i2 = Interval::new(Moment::new(23, 00, 00), Moment::new(01, 00, 00));
        assert_eq!(i2.from, Moment::new(01, 00, 00));
        assert_eq!(i2.to, Moment::new(23, 00, 00));
    }
}