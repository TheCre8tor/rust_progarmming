mod i1_drop;

fn main() {
    /* Summary of utility traits ->
      1. Drop -> Destructors. Cleanup code that Rust runs
         automatically whenever a value is dropped.
      2. Sized -> Marker trait for types with a fixed size
         known at compile time, as opposed to types
         (such as slices) that  are dynamically sized.
      3. Clone -> Types that support cloning values.
      4. Copy -> Marker trait for types that can be cloned
         simply by making a byte-for-byte copy of the
         memory containing a value.
      5. Deref and DerefMut -> Traits for smart pointer value.
      6. Default -> Types that have a sensible "default value."
      7. AsRef and AsMut -> Conversion traits for borrowing one
         type of reference from another.
      8. Borrow and BorrowMut -> Conversion traits, like AsRef/
         AsMut, but additionally guaranteeing consistent hashing,
         ordering, and equality.
      9. From and Into -> Conversion traits for tranforming one
         type of value into another.
      10. ToOwned -> Conversion trait for converting a reference
          to an owned value.

    */
    i1_drop::run();
}
