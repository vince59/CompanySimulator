use common::{
    self,
    time_simulator::{SimDateTime, TimeScale, Timer},
};

#[derive(Debug)]
pub struct SimulatorParameters {
    pub time_scale: TimeScale,
    pub initial_date_time: SimDateTime,
    pub elapsed_simulation_time: u64, // in seconds
}

impl Default for SimulatorParameters {
    fn default() -> Self {
        SimulatorParameters {
            time_scale: TimeScale::OneHourPerSecond,
            initial_date_time: SimDateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
            },
            elapsed_simulation_time: 1200, // 20 minutes in seconds
        }
    }
}

pub struct SimulatorState {
    pub timer: Timer,
}

impl SimulatorState {
    pub fn new(parameters: SimulatorParameters) -> Self {
        let timer = Timer::new(
            parameters.time_scale,
            parameters.initial_date_time,
            parameters.elapsed_simulation_time,
        );
        SimulatorState { timer }
    }
}
