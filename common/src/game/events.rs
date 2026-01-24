use crate::game::company::Company;
use crate::time_simulator::scheduler::{EventAction, EventOutcome, EventRecurrence};
#[derive(Debug)]
pub struct SalaryPaymentEvent {
    pub recurrence: EventRecurrence,
    done: bool, // utile pour Once
}

impl SalaryPaymentEvent {
    pub fn new(recurrence: EventRecurrence) -> Self {
        Self {
            recurrence,
            done: false,
        }
    }
}

impl EventAction for SalaryPaymentEvent {
    fn name(&self) -> &'static str {
        "SalaryPayment"
    }

    fn recurrence(&self) -> &EventRecurrence {
        &self.recurrence
    }

    fn trigger(&mut self, company: &mut Company) -> EventOutcome {
        let payroll: f64 = company
            .services
            .iter()
            .flat_map(|s| s.employees.iter())
            .map(|e| e.salary)
            .sum();

        if payroll <= 0.0 {
            return EventOutcome::Skipped("Payroll is zero");
        }

        company.cash_balance -= payroll;

        if matches!(self.recurrence, EventRecurrence::Once) {
            self.done = true;
        }

        EventOutcome::Applied { amount: payroll }
    }

    fn is_done(&self) -> bool {
        self.done
    }
}


