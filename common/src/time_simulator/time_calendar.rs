use chrono::{DateTime, Duration, TimeZone, Utc,Datelike,Timelike};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarDelta {
    pub years: i32,
    pub months: i32,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
}

fn days_in_month(year: i32, month: u32) -> u32 {
    // month: 1..=12
    let first_next = if month == 12 {
        Utc.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0).unwrap()
    } else {
        Utc.with_ymd_and_hms(year, month + 1, 1, 0, 0, 0).unwrap()
    };
    let last_this = first_next - Duration::days(1);
    last_this.day()
}

fn add_years_months_clamped(dt: DateTime<Utc>, years: i32, months: i32) -> DateTime<Utc> {
    // Ajoute years+months, en "clampant" le jour si nécessaire (ex: 31 -> 30/28)
    let mut y = dt.year();
    let mut m = dt.month() as i32; // 1..=12

    // années
    y += years;

    // mois
    m += months;
    while m > 12 {
        m -= 12;
        y += 1;
    }
    while m < 1 {
        m += 12;
        y -= 1;
    }

    let max_day = days_in_month(y, m as u32);
    let day = dt.day().min(max_day);

    Utc.with_ymd_and_hms(
        y,
        m as u32,
        day,
        dt.hour(),
        dt.minute(),
        dt.second(),
    )
    .unwrap()
}

/// Différence calendaire (années/mois/jours/heures/min/sec) entre deux dates.
/// Si end < start, la fonction inverse et te donne le delta en valeur absolue.
/// (Si tu veux garder le signe, je te le fais aussi.)
pub fn calendar_diff(start: DateTime<Utc>, end: DateTime<Utc>) -> CalendarDelta {
    let (start, end) = if end >= start { (start, end) } else { (end, start) };

    // 1) approx années/mois via composantes
    let mut years = end.year() - start.year();
    let mut months = end.month() as i32 - start.month() as i32;
    if months < 0 {
        months += 12;
        years -= 1;
    }

    // 2) on “applique” (years, months) au start, et si on dépasse end, on recule d’un mois
    let mut anchor = add_years_months_clamped(start, years, months);
    if anchor > end {
        // reculer d'un mois
        months -= 1;
        if months < 0 {
            months += 12;
            years -= 1;
        }
        anchor = add_years_months_clamped(start, years, months);
    }

    // 3) reste = end - anchor => jours/heures/min/sec
    let mut rem = end - anchor;

    let days = rem.num_days();
    rem = rem - Duration::days(days);

    let hours = rem.num_hours();
    rem = rem - Duration::hours(hours);

    let minutes = rem.num_minutes();
    rem = rem - Duration::minutes(minutes);

    let seconds = rem.num_seconds();

    CalendarDelta {
        years,
        months,
        days,
        hours,
        minutes,
        seconds,
    }
}
