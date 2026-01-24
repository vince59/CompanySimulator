#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeScale {
    RealTime,
    OneMinutePerSecond,
    OneHourPerSecond,
    EightHoursPerSecond,
    OneDayPerSecond,
    OneMonthPerSecond,
}

impl TimeScale {
    pub fn seconds_multiplier(self) -> f64 {
        match self {
            TimeScale::RealTime => 1.0,
            TimeScale::OneMinutePerSecond => 60.0,
            TimeScale::OneHourPerSecond => 3600.0,
            TimeScale::EightHoursPerSecond => 8.0 * 3600.0,
            TimeScale::OneDayPerSecond => 24.0 * 3600.0,
            TimeScale::OneMonthPerSecond => 30.0 * 24.0 * 3600.0,
        }
    }
}

#[derive(Debug)]
pub struct SimDateTime {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

