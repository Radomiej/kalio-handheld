use serde::{Deserialize, Serialize};

// ── Input ──────────────────────────────────────────────────────────────────

/// Logical game actions, independent of physical hardware.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputAction {
    Confirm,
    Cancel,
    Up,
    Down,
    Left,
    Right,
    Menu,
    Select,
    L1,
    R1,
    L2,
    R2,
}

// ── Color ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE:       Self = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK:       Self = Color { r: 0,   g: 0,   b: 0,   a: 255 };
    pub const RED:         Self = Color { r: 255, g: 0,   b: 0,   a: 255 };
    pub const GREEN:       Self = Color { r: 0,   g: 255, b: 0,   a: 255 };
    pub const BLUE:        Self = Color { r: 0,   g: 0,   b: 255, a: 255 };
    pub const TRANSPARENT: Self = Color { r: 0,   g: 0,   b: 0,   a: 0   };

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

// ── Render commands ────────────────────────────────────────────────────────

/// Command-buffer renderer — widgets produce these, SDL2 executes them.
#[derive(Debug, Clone)]
pub enum RenderCommand {
    Clear { color: Color },
    Rect  { x: i32, y: i32, w: u32, h: u32, color: Color },
    Text  { x: i32, y: i32, text: String, size: u16, color: Color },
    Image { x: i32, y: i32, w: u32, h: u32, path: String },
}

// ── App config ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name:    String,
    pub version: String,
    pub width:   u32,
    pub height:  u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name:    "Kalio".into(),
            version: "0.1.0".into(),
            width:   640,
            height:  480,
        }
    }
}
