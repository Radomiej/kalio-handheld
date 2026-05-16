use anyhow::Result;
use kalio_shared::{AppConfig, Color, RenderCommand};
use sdl2::event::Event;
use tracing::info;

use crate::{
    input::InputSystem,
    layout::LayoutEngine,
    renderer::Renderer,
    scripting::ScriptEngine,
    storage::Storage,
};

pub fn run() -> Result<()> {
    // ── SDL2 init ──
    let sdl     = sdl2::init().map_err(|e| anyhow::anyhow!(e))?;
    let video   = sdl.video().map_err(|e| anyhow::anyhow!(e))?;

    let config  = load_config();

    let window  = video
        .window(&config.name, config.width, config.height)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let canvas  = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let mut renderer = Renderer::new(canvas);

    // ── Controllers ──
    let gc   = sdl.game_controller().map_err(|e| anyhow::anyhow!(e))?;
    let mut controllers = Vec::new();
    for i in 0..gc.num_joysticks().unwrap_or(0) {
        if gc.is_game_controller(i) {
            if let Ok(c) = gc.open(i) {
                info!("Controller found: {}", c.name());
                controllers.push(c);
            }
        }
    }
    let mut input = InputSystem::new(controllers);

    // ── Subsystems ──
    let _layout  = LayoutEngine::new();
    let storage  = Storage::open_in_memory()?;
    storage.set("runtime.version", env!("CARGO_PKG_VERSION"))?;

    let script = ScriptEngine::new()?;
    script.register_api()?;
    script.exec_str(r#"log("Kalio Lua 5.4 ready")"#)?;

    let mut event_pump = sdl.event_pump().map_err(|e| anyhow::anyhow!(e))?;
    info!("Entering main loop ({}x{})", config.width, config.height);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                e => input.process_event(&e),
            }
        }

        let cmds = build_ui(input.state());
        renderer.execute(&cmds);
    }

    info!("Runtime shutdown clean");
    Ok(())
}

fn load_config() -> AppConfig {
    std::fs::read_to_string("config/default.ron")
        .ok()
        .and_then(|s| ron::from_str(&s).ok())
        .unwrap_or_default()
}

fn build_ui(input: &crate::input::InputState) -> Vec<RenderCommand> {
    let last_action = input
        .actions_just_pressed
        .first()
        .map(|a| format!("{a:?}"))
        .unwrap_or_else(|| "none".into());

    vec![
        RenderCommand::Clear { color: Color::new(15, 15, 25, 255) },
        RenderCommand::Rect {
            x: 20, y: 20, w: 240, h: 54,
            color: Color::new(50, 100, 180, 255),
        },
        RenderCommand::Text {
            x: 32, y: 32,
            text: "Kalio Runtime".into(),
            size: 20,
            color: Color::WHITE,
        },
        RenderCommand::Text {
            x: 20, y: 100,
            text: format!("Last action: {last_action}"),
            size: 14,
            color: Color::new(180, 180, 180, 255),
        },
        RenderCommand::Text {
            x: 20, y: 124,
            text: format!("Held: {} actions", input.actions_held.len()),
            size: 14,
            color: Color::new(140, 140, 160, 255),
        },
    ]
}
