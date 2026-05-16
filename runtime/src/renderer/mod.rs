use kalio_shared::{Color, RenderCommand};
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use tracing::debug;

pub struct Renderer {
    canvas: Canvas<Window>,
}

impl Renderer {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Self { canvas }
    }

    pub fn execute(&mut self, commands: &[RenderCommand]) {
        for cmd in commands {
            self.draw(cmd);
        }
        self.canvas.present();
    }

    fn draw(&mut self, cmd: &RenderCommand) {
        match cmd {
            RenderCommand::Clear { color } => {
                self.canvas.set_draw_color(sdl(*color));
                self.canvas.clear();
            }
            RenderCommand::Rect { x, y, w, h, color } => {
                self.canvas.set_draw_color(sdl(*color));
                let _ = self.canvas.fill_rect(Rect::new(*x, *y, *w, *h));
            }
            RenderCommand::Text { x, y, text, size, color } => {
                // TTF text rendering — implement with font cache in next iteration
                debug!("[text] '{text}' @ ({x},{y}) sz={size} col={color:?}");
            }
            RenderCommand::Image { x, y, w, h, path } => {
                debug!("[img]  '{path}' @ ({x},{y}) {w}x{h}");
            }
        }
    }
}

#[inline]
fn sdl(c: Color) -> SdlColor {
    SdlColor::RGBA(c.r, c.g, c.b, c.a)
}
