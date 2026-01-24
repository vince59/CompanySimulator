use common::{
    self,
    game::{
        Company,
        company::{Employee, EmployeeType, EventScheduler, Service, ServiceType},
    },
    time_simulator::{SimDateTime, TimeScale, Timer},
};

#[derive(Debug)]
pub struct SimulatorParameters {
    pub time_scale: TimeScale,
    pub initial_date_time: SimDateTime,
    /// in seconds
    pub elapsed_simulation_time: u64,
    pub company: Company,
}

fn employee(name: &str, employee_type: EmployeeType, salary: f64) -> Employee {
    Employee {
        name: name.to_string(),
        employee_type,
        salary,
    }
}

fn initialize_services() -> Vec<Service> {
    use EmployeeType::*;
    use ServiceType::*;

    let mut services = Vec::new();

    let all_services = [
        Logistics,
        Marketing,
        Development,
        ITDepartment,
        Sales,
        Purchasing,
        Administration,
        Production,
    ];

    for service_type in all_services {
        let mut employees = Vec::new();

        // 1 manager par service
        employees.push(employee("Manager", Manager, 5_000.0));

        // 5 technicians par service
        for i in 1..=5 {
            employees.push(employee(&format!("Technician {}", i), Technician, 3_000.0));
        }

        // Spécificités par service
        match service_type {
            Development => {
                for i in 1..=3 {
                    employees.push(employee(&format!("Engineer {}", i), Engineer, 4_000.0));
                }
            }
            ITDepartment => {
                for i in 1..=2 {
                    employees.push(employee(&format!("Engineer {}", i), Engineer, 4_200.0));
                }
            }
            Sales => {
                for i in 1..=4 {
                    employees.push(employee(
                        &format!("Salesperson {}", i),
                        Salesperson,
                        3_500.0,
                    ));
                }
            }
            Production => {
                for i in 1..=10 {
                    employees.push(employee(&format!("Worker {}", i), Worker, 2_500.0));
                }
            }
            _ => {}
        }

        services.push(Service {
            service_type,
            employees,
        });
    }

    services
}

impl Default for SimulatorParameters {
    fn default() -> Self {
        SimulatorParameters {
            time_scale: TimeScale::OneMonthPerSecond,
            initial_date_time: SimDateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
            },
            elapsed_simulation_time: 1200,
            company: Company {
                name: "Default Company".to_string(),
                cash_balance: 10_000_000.0,
                services: initialize_services(),
            },
        }
    }
}

pub struct SimulatorState {
    pub timer: Timer,
    pub company: Company,
    pub event_scheduler: EventScheduler,
}

impl SimulatorState {
    pub fn new(parameters: SimulatorParameters) -> Self {
        let timer = Timer::new(
            parameters.time_scale,
            parameters.initial_date_time,
            parameters.elapsed_simulation_time,
        );
        SimulatorState {
            timer,
            company: parameters.company,
            event_scheduler: EventScheduler::new(),
        }
    }
}
