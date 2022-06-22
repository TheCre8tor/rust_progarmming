#[path = "./i3_passing_ref_as_args.rs"]
mod passing_ref_as_args;

#[path = "./i4_returning_references.rs"]
mod returning_references;

#[path = "./i5_structs_containing_references.rs"]
mod structs_containing_references;

#[path = "./i6_distinct_lifetime_parameters.rs"]
mod distinct_lifetime_parameters;

fn main() {
    /* Rust’s equivalent of a global variable is called
       a static: it’s a value that’s created when the
       program starts and lasts until it terminates.

       RULES:
       1. Every static must be initialized.
       2. Mutable statics are inherently not thread-safe.
          -> For these reasons, you may access a mutable
          static only within an unsafe block.
    */
    static mut STASH: &i32 = &128;

    static WORTH_POINTING_AT: i32 = 3;

    fn f(p: &'static i32) {
        unsafe {
            STASH = p;
        }
    }

    f(&WORTH_POINTING_AT);

    // LECTURE: 2
    passing_ref_as_args::run();
    returning_references::run();
    structs_containing_references::run();
    distinct_lifetime_parameters::run();
}
