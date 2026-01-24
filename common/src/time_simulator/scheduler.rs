use chrono::{DateTime, Datelike, Utc};
use crate::game::company::{Company,SalaryPaymentEvent};
pub enum EventType {
    SalaryPayment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventRecurrence {
    EndOfMonth,
    EndOfYear,
    Quarterly,
    Monthly,
    Yearly,
    EveryNMonths(u8),
    Once,
}

impl EventRecurrence {
    /// Retourne true si l'intervalle [prev, now] franchit une "frontière"
    /// correspondant à la récurrence (changement de mois, trimestre, année, etc.)
    pub fn should_trigger(&self, prev: DateTime<Utc>, now: DateTime<Utc>) -> bool {
        if now <= prev {
            return false;
        }

        match self {
            EventRecurrence::Once => false, // déclenché manuellement ou via flag `done`

            EventRecurrence::Monthly | EventRecurrence::EndOfMonth => {
                (prev.year(), prev.month()) != (now.year(), now.month())
            }

            EventRecurrence::Yearly | EventRecurrence::EndOfYear => prev.year() != now.year(),

            EventRecurrence::Quarterly => {
                let prev_q = (prev.year(), prev.month0() / 3);
                let now_q = (now.year(), now.month0() / 3);
                prev_q != now_q
            }

            EventRecurrence::EveryNMonths(n) => {
                let n = *n as i32;
                if n <= 0 {
                    return false;
                }

                // index de mois croissant (année * 12 + mois)
                let prev_idx = prev.year() * 12 + prev.month() as i32; // mois: 1..=12
                let now_idx = now.year() * 12 + now.month() as i32;

                (prev_idx / n) != (now_idx / n)
            }
        }
    }
}

pub struct Event {
    pub event_type: EventType,
    pub recurrence: EventRecurrence,
}

#[derive(Debug)]
pub enum EventOutcome {
    Applied { amount: f64 },
    Skipped(&'static str),
}

pub trait EventAction: std::fmt::Debug + Send + Sync {
    fn name(&self) -> &'static str;
    fn recurrence(&self) -> &EventRecurrence;

    /// Contenu spécifique de l'évènement
    fn trigger(&mut self, company: &mut Company) -> EventOutcome;

    /// Optionnel : si tu veux gérer "Once"
    fn is_done(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct EventScheduler {
    last_sim_time: Option<DateTime<Utc>>,
    events: Vec<Box<dyn EventAction>>,
}

impl EventScheduler {
    pub fn new() -> Self {
        Self {
            last_sim_time: None,
            events: Vec::new(),
        }
    }

    pub fn add(&mut self, event: Box<dyn EventAction>) {
        self.events.push(event);
    }

    /// À appeler une fois par tick de simulation.
    pub fn tick(&mut self, current_sim_time: DateTime<Utc>, company: &mut Company) {
        let prev = match self.last_sim_time {
            None => {
                self.last_sim_time = Some(current_sim_time);
                return;
            }
            Some(t) => t,
        };

        if current_sim_time <= prev {
            self.last_sim_time = Some(current_sim_time);
            return;
        }

        for e in self.events.iter_mut() {
            if e.recurrence().should_trigger(prev, current_sim_time) {
                let outcome = e.trigger(company);
                println!(
                    "[{}] {} => {:?} (cash={})",
                    current_sim_time,
                    e.name(),
                    outcome,
                    company.cash_balance
                );
            }
        }

        // Nettoie les events Once
        self.events.retain(|e| !e.is_done());

        self.last_sim_time = Some(current_sim_time);
    }

    pub fn init_default_events(&mut self) {
        // Paiement des salaires à la fin de chaque mois
        let salary_event = SalaryPaymentEvent::new(EventRecurrence::EndOfMonth);
        self.add(Box::new(salary_event));
    }

}