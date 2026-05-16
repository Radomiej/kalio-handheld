use kalio_shared::InputAction;
use sdl2::controller::{Button, GameController};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive(Debug, Default)]
pub struct InputState {
    pub actions_just_pressed:  Vec<InputAction>,
    pub actions_held:          Vec<InputAction>,
    pub actions_just_released: Vec<InputAction>,
}

pub struct InputSystem {
    state:       InputState,
    _controllers: Vec<GameController>,
}

impl InputSystem {
    pub fn new(controllers: Vec<GameController>) -> Self {
        Self {
            state:       InputState::default(),
            _controllers: controllers,
        }
    }

    pub fn process_event(&mut self, event: &Event) {
        self.state.actions_just_pressed.clear();
        self.state.actions_just_released.clear();

        match event {
            // ── Keyboard → gamepad (desktop-debug) ────────────────────────
            #[cfg(feature = "desktop-debug")]
            Event::KeyDown { keycode: Some(kc), repeat: false, .. } => {
                if let Some(a) = key_to_action(*kc) {
                    if !self.state.actions_held.contains(&a) {
                        self.state.actions_held.push(a.clone());
                    }
                    self.state.actions_just_pressed.push(a);
                }
            }
            #[cfg(feature = "desktop-debug")]
            Event::KeyUp { keycode: Some(kc), .. } => {
                if let Some(a) = key_to_action(*kc) {
                    self.state.actions_held.retain(|x| x != &a);
                    self.state.actions_just_released.push(a);
                }
            }

            // ── GameController ────────────────────────────────────────────
            Event::ControllerButtonDown { button, .. } => {
                if let Some(a) = btn_to_action(*button) {
                    if !self.state.actions_held.contains(&a) {
                        self.state.actions_held.push(a.clone());
                    }
                    self.state.actions_just_pressed.push(a);
                }
            }
            Event::ControllerButtonUp { button, .. } => {
                if let Some(a) = btn_to_action(*button) {
                    self.state.actions_held.retain(|x| x != &a);
                    self.state.actions_just_released.push(a);
                }
            }
            _ => {}
        }
    }

    pub fn state(&self) -> &InputState {
        &self.state
    }
}

#[cfg(feature = "desktop-debug")]
fn key_to_action(kc: Keycode) -> Option<InputAction> {
    // WASD / arrows → d-pad  |  J/Enter → A  |  K/Esc → B  |  U → X  |  I → Y
    Some(match kc {
        Keycode::W | Keycode::Up    => InputAction::Up,
        Keycode::S | Keycode::Down  => InputAction::Down,
        Keycode::A | Keycode::Left  => InputAction::Left,
        Keycode::D | Keycode::Right => InputAction::Right,
        Keycode::J | Keycode::Return => InputAction::Confirm,
        Keycode::K | Keycode::Escape => InputAction::Cancel,
        Keycode::U                  => InputAction::Select,
        Keycode::I                  => InputAction::Menu,
        Keycode::Q                  => InputAction::L1,
        Keycode::E                  => InputAction::R1,
        _ => return None,
    })
}

fn btn_to_action(btn: Button) -> Option<InputAction> {
    Some(match btn {
        Button::A            => InputAction::Confirm,
        Button::B            => InputAction::Cancel,
        Button::X            => InputAction::Select,
        Button::Y            => InputAction::Menu,
        Button::DPadUp       => InputAction::Up,
        Button::DPadDown     => InputAction::Down,
        Button::DPadLeft     => InputAction::Left,
        Button::DPadRight    => InputAction::Right,
        Button::Start        => InputAction::Menu,
        Button::LeftShoulder  => InputAction::L1,
        Button::RightShoulder => InputAction::R1,
        _ => return None,
    })
}
