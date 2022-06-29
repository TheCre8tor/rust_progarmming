use std::mem::size_of;

#[derive(Debug)]
enum Pet {
    Orca = 29,
    Giraffe,
}

enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
    ServerError = 500,
}

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

/// A timestamp that has been deliberately rounded off, so our program
/// says "6 months ago" instead of "February 9, 2016, at 9:49 AM".
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32), // this is called a turple variants
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn main() {
    fn mar(val: i8) -> Pet {
        if val < 2 {
            Pet::Orca
        } else {
            Pet::Giraffe
        }
    }

    println!("{:?}", mar(-2));

    /* In memory, values of C-style enums are stored as integers.
       Occasionally it’s useful to tell Rust which integers to use
       Otherwise Rust will assign the numbers for you, starting at 0.
    */

    // As of Rust 1.17, RoughTime fits in 8 bytes
    assert_eq!(size_of::<RoughTime>(), 8);

    // EXAMPLE 2 ->
    let word = TimeUnit::Days;
    let singular = word.singular();
    let plural = word.plural();

    println!("Singular: {singular} | Plural: {plural}");
    collect(word);

    // EXAMPLE 3 ->
    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    println!("{:?}", four_score_and_seven_years_ago);
    println!("{:?}", three_hours_from_now);

    /* Rust 🦀 has three kinds of enum variant.
       1. Variants with no data correspond to unit-like structs.
       2. Tuple variants look and function just like tuple structs.
       3. Struct variants have curly braces and named fields.
    */
}

fn collect(data: TimeUnit) {
    println!("Data: {:?}", data);
}
