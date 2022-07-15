use std::io::Write;
mod i1_traits_and_peoples_types;

fn main() {
    i1_traits_and_peoples_types::run();
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Clone, Copy)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

type Canvas = Box<dyn Write>;

/// A trait for characters, items, and scenery -
/// anything in the game world that's visible on screen.
trait Visible {
    /// Render this object on the given canvas.
    fn draw(&self, canvas: &mut Canvas);

    /// Return true if clicking at (x, y) should
    /// select this object.
    fn hit_test(&self, x: i32, y: i32);
}

impl Broom {
    // Helper function used by Broom::draw() below.
    // fn broomstick_range(&self) -> Range<i32> {}
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        // for y in self.broomstick_range() {}
        todo!()
    }

    fn hit_test(&self, x: i32, y: i32) {
        todo!()
    }
}
