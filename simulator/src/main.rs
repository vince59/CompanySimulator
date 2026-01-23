mod simulator;
use simulator::param::{SimulatorParameters, SimulatorState};

fn main() {
    println!("Running the Company Simulator! Version 1.0");
    let parameters = SimulatorParameters::default();
     println!(
        "Simulator initialized with parameters: {:?}",
        parameters
    );
    let mut simulator_state = SimulatorState::new(parameters);
   
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!(
            "Simulated time: {} (Elapsed simulation time: {})",
            simulator_state.timer.get_formated_simulated_time(),
            simulator_state.timer.get_formated_elapsed_simulation_time()
        );
    }
}
