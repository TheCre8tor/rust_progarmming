#[derive(Copy, Clone, Debug, PartialEq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn main() {
    let _four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    let _three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    let a_month_from_now = RoughTime::InTheFuture(TimeUnit::Months, 1);
    let _just_now = RoughTime::JustNow;

    let print_date = rough_time_to_english(a_month_from_now);
    println!("{}", print_date);
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => {
            format!("{} {} ago", count, units.plural())
        }
        RoughTime::JustNow => {
            format!("just now")
        }
        RoughTime::InTheFuture(units, 1) => {
            format!("a {} from now", units.singular())
        }
        RoughTime::InTheFuture(units, count) => {
            format!("{} {} from now", count, units.plural())
        }
    }
}
