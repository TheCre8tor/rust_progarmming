use std::io::Write;

fn main() {}

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

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        todo!()
    }

    fn hit_test(&self, x: i32, y: i32) {
        todo!()
    }
}
