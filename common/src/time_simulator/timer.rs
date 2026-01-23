use chrono::{DateTime, Duration, TimeZone, Utc};
use std::time::Instant;
use crate::time_simulator::{
    time_calendar::calendar_diff,
    time_scale::{TimeScale, SimDateTime},
};


pub struct Timer {
    time_scale: TimeScale,
    simulated_time: DateTime<Utc>,
    initial_date_time: DateTime<Utc>,
    last_tick: Instant,
    /// in seconds
    elapsed_simulation_time: u64, 
}

impl Timer {
    pub fn new(
        time_scale: TimeScale,
        initial_date_time: SimDateTime,
        elapsed_simulation_time: u64,
    ) -> Self {
        Timer {
            time_scale,
            last_tick: Instant::now(),
            initial_date_time: Utc
                .with_ymd_and_hms(
                    initial_date_time.year,
                    initial_date_time.month.into(),
                    initial_date_time.day.into(),
                    initial_date_time.hour.into(),
                    initial_date_time.minute.into(),
                    initial_date_time.second.into(),
                )
                .unwrap(),
            simulated_time: Utc
                .with_ymd_and_hms(
                    initial_date_time.year,
                    initial_date_time.month.into(),
                    initial_date_time.day.into(),
                    initial_date_time.hour.into(),
                    initial_date_time.minute.into(),
                    initial_date_time.second.into(),
                )
                .unwrap()
                + Duration::seconds(elapsed_simulation_time as i64),
            elapsed_simulation_time,
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let real_dt = now.duration_since(self.last_tick);
        self.last_tick = now;

        // Convert real elapsed to simulated elapsed
        let real_secs = real_dt.as_secs_f64();
        let sim_secs = real_secs * self.time_scale.seconds_multiplier();
        self.simulated_time = self.simulated_time + Duration::seconds(sim_secs as i64);
        self.elapsed_simulation_time += sim_secs as u64;
    }

    pub fn get_formated_simulated_time(&mut self) -> String {
        self.tick();
        self.simulated_time.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn get_elapsed_simulation_time(&mut self) -> u64 {
        self.tick();
        self.elapsed_simulation_time
    }

    pub fn get_formated_elapsed_simulation_time(&mut self) -> String {
        self.tick();
        let simulated_time = calendar_diff(self.initial_date_time, self.simulated_time);
        format!(
            "{} years, {} months, {} days, {:02}h:{:02}m:{:02}s",
            simulated_time.years,
            simulated_time.months,
            simulated_time.days,
            simulated_time.hours,
            simulated_time.minutes,
            simulated_time.seconds
        )
    }
}
