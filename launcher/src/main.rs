//! Kalio Launcher — top-level UI app that runs on the Kalio Runtime.
//!
//! Launch via the runtime binary; this crate provides the launcher logic
//! as a library that the runtime can load.  For now it is a standalone
//! demo binary so you can run `cargo run -p kalio-launcher` directly.

use kalio_sdk::{AppContext, Color, InputAction, KalioApp};

struct LauncherApp {
    selected: usize,
    apps: Vec<&'static str>,
}

impl KalioApp for LauncherApp {
    fn on_start(&mut self, ctx: &mut AppContext) {
        ctx.clear(Color::new(10, 10, 20, 255));
    }

    fn on_update(&mut self, ctx: &mut AppContext) {
        ctx.clear(Color::new(10, 10, 20, 255));

        // Title
        ctx.text(20, 16, "Kalio Launcher", 22, Color::WHITE);

        // App list
        for (i, name) in self.apps.iter().enumerate() {
            let y = 60 + (i as i32) * 40;
            let bg = if i == self.selected {
                Color::new(60, 120, 200, 255)
            } else {
                Color::new(30, 30, 45, 255)
            };
            ctx.rect(16, y, 300, 32, bg);
            ctx.text(24, y + 8, *name, 16, Color::WHITE);
        }

        // Navigation
        if ctx.pressed(&InputAction::Up) && self.selected > 0 {
            self.selected -= 1;
        }
        if ctx.pressed(&InputAction::Down) && self.selected + 1 < self.apps.len() {
            self.selected += 1;
        }
    }
}

fn main() {
    println!("Kalio Launcher (standalone preview)");
    let mut app = LauncherApp {
        selected: 0,
        apps:     vec!["Hello World", "Settings", "File Manager"],
    };
    let mut ctx = AppContext::new();
    app.on_start(&mut ctx);
    app.on_update(&mut ctx);
    println!("Render commands: {}", ctx.commands.len());
}
