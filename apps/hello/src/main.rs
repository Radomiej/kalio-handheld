use kalio_sdk::{AppContext, Color, InputAction, KalioApp};

struct HelloApp {
    frame:   u64,
    counter: i32,
}

impl KalioApp for HelloApp {
    fn on_start(&mut self, ctx: &mut AppContext) {
        ctx.clear(Color::new(10, 10, 20, 255));
        println!("Hello App started!");
    }

    fn on_update(&mut self, ctx: &mut AppContext) {
        self.frame += 1;

        ctx.clear(Color::new(10, 10, 20, 255));
        ctx.rect(20, 20, 240, 56, Color::new(50, 100, 180, 255));
        ctx.text(30, 34, "Hello, Kalio!", 20, Color::WHITE);
        ctx.text(20, 94, format!("Frame: {}", self.frame), 14, Color::new(180, 180, 200, 255));
        ctx.text(20, 114, format!("Counter: {}", self.counter), 14, Color::new(180, 200, 180, 255));

        if ctx.pressed(&InputAction::Confirm) {
            self.counter += 1;
        }
        if ctx.pressed(&InputAction::Cancel) {
            self.counter -= 1;
        }
    }
}

fn main() {
    println!("Hello App (standalone preview — no SDL2 window)");
    let mut app = HelloApp { frame: 0, counter: 0 };
    let mut ctx = AppContext::new();
    app.on_start(&mut ctx);

    // Simulate a few frames
    for _ in 0..3 {
        app.on_update(&mut ctx);
    }
    println!("{} render commands after 3 frames", ctx.commands.len());
}
