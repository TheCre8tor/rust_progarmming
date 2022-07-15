trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for &str {
    fn is_emoji(&self) -> bool {
        !self.is_empty()
    }
}

pub fn run() {
    /* Extension Trait ->
       The sole purpose of this particular trait is to
       add a method to an existing type, char.

       Like any other trait method, this new is_emoji
       method is only visible when IsEmoji is in scope.
    */
    assert_eq!("ddd".is_emoji(), false);
}
