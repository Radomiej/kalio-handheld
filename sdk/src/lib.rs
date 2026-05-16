pub use kalio_shared::{AppConfig, Color, InputAction, RenderCommand};

// ── App trait ─────────────────────────────────────────────────────────────

pub trait KalioApp {
    fn on_start(&mut self, _ctx: &mut AppContext) {}
    fn on_update(&mut self, ctx: &mut AppContext);
    fn on_stop(&mut self, _ctx: &mut AppContext) {}
}

// ── App context ──────────────────────────────────────────────────────────

#[derive(Debug, Default, Clone)]
pub struct InputSnapshot {
    pub just_pressed:  Vec<InputAction>,
    pub held:          Vec<InputAction>,
    pub just_released: Vec<InputAction>,
}

pub struct AppContext {
    pub commands: Vec<RenderCommand>,
    pub input:    InputSnapshot,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            input:    InputSnapshot::default(),
        }
    }

    // ── Drawing helpers ──

    pub fn clear(&mut self, color: Color) {
        self.commands.push(RenderCommand::Clear { color });
    }

    pub fn rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        self.commands.push(RenderCommand::Rect { x, y, w, h, color });
    }

    pub fn text(&mut self, x: i32, y: i32, text: impl Into<String>, size: u16, color: Color) {
        self.commands.push(RenderCommand::Text {
            x, y,
            text: text.into(),
            size,
            color,
        });
    }

    // ── Input helpers ──

    pub fn pressed(&self, action: &InputAction) -> bool {
        self.input.just_pressed.contains(action)
    }

    pub fn held(&self, action: &InputAction) -> bool {
        self.input.held.contains(action)
    }

    pub fn released(&self, action: &InputAction) -> bool {
        self.input.just_released.contains(action)
    }
}

impl Default for AppContext {
    fn default() -> Self { Self::new() }
}
