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

    // The size of HttpStatus with its variants altogether is 16bit.
    assert_eq!(size_of::<HttpStatus>(), 2);

    // EXAMPLE 2 ->
    let word = TimeUnit::Days;
    let singular = word.singular();
    let plural = word.plural();

    println!("Singular: {singular} | Plural: {plural}");
    collect(word);

    // EXAMPLE 3 ->
}

fn collect(data: TimeUnit) {
    println!("Data: {:?}", data);
}
