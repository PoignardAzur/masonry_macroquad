use keyboard_types::Modifiers;
use masonry_core::{
    app::RenderRoot,
    core::{KeyboardEvent, TextEvent, WindowEvent},
    dpi::{PhysicalPosition, PhysicalSize},
};
use miniquad::*;
use ui_events::{
    ScrollDelta,
    keyboard::{Code, Key, KeyState, Location, NamedKey},
    pointer::{
        PointerButtonEvent, PointerEvent, PointerInfo, PointerScrollEvent, PointerState,
        PointerType, PointerUpdate,
    },
};

/// Per-Window state
pub struct Window {
    // id: WindowId,
    // pub(crate) handle: Arc<WindowHandle>,
    // pub(crate) accesskit_adapter: Adapter,
    // event_reducer: WindowEventReducer,
    pub(crate) render_root: RenderRoot,
    pub(crate) pointer_state: PointerState,
    pub(crate) backend: Box<dyn miniquad::RenderingBackend>,
    // pub(crate) base_color: Color,
}

pub(crate) struct Image {
    pub(crate) bytes: Vec<u8>,
    pub(crate) width: u16,
    pub(crate) height: u16,
}

impl EventHandler for Window {
    fn update(&mut self) {
        todo!()
    }

    fn draw(&mut self) {
        let image: Image = self.get_image();

        // TODO
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        let size = PhysicalSize::new(width as u32, height as u32);
        self.render_root
            .handle_window_event(WindowEvent::Resize(size));
        self.handle_signals();
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        // TODO - Check how this handles non-identity scale factors.
        self.pointer_state.position = PhysicalPosition::new(x as f64, y as f64);

        let event = PointerEvent::Move(self.pointer_update());
        self.render_root.handle_pointer_event(event);
        self.handle_signals();
    }

    fn mouse_wheel_event(&mut self, delta_x: f32, delta_y: f32) {
        let event = PointerEvent::Scroll(PointerScrollEvent {
            pointer: self.pointer_info(),
            delta: ScrollDelta::PixelDelta(PhysicalPosition::new(delta_x as f64, delta_y as f64)),
            state: self.pointer_state.clone(),
        });
        self.render_root.handle_pointer_event(event);
        self.handle_signals();
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        let button = convert_mouse_button(button);

        // TODO - Check how this handles non-identity scale factors.
        self.pointer_state.position = PhysicalPosition::new(x as f64, y as f64);
        if let Some(button) = button {
            self.pointer_state.buttons.insert(button);
        }

        let event = PointerEvent::Down(PointerButtonEvent {
            button,
            pointer: self.pointer_info(),
            state: self.pointer_state.clone(),
        });
        self.render_root.handle_pointer_event(event);
        self.handle_signals();
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
        let button = convert_mouse_button(button);

        // TODO - Check how this handles non-identity scale factors.
        self.pointer_state.position = PhysicalPosition::new(x as f64, y as f64);
        if let Some(button) = button {
            self.pointer_state.buttons.remove(button);
        }

        let event = PointerEvent::Up(PointerButtonEvent {
            button,
            pointer: self.pointer_info(),
            state: self.pointer_state.clone(),
        });
        self.render_root.handle_pointer_event(event);
        self.handle_signals();
    }

    fn char_event(&mut self, _character: char, _keymods: KeyMods, _repeat: bool) {
        //
    }

    fn key_down_event(&mut self, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        let event = TextEvent::Keyboard(convert_keycode(KeyState::Down, keycode, keymods, repeat));

        self.render_root.handle_text_event(event);
        self.handle_signals();
    }

    fn key_up_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
        let event = TextEvent::Keyboard(convert_keycode(KeyState::Up, keycode, keymods, false));

        self.render_root.handle_text_event(event);
        self.handle_signals();
    }

    fn touch_event(&mut self, phase: TouchPhase, _id: u64, x: f32, y: f32) {
        // TODO - Replace with a proper implementation.
        if phase == TouchPhase::Started {
            self.mouse_button_down_event(MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Ended {
            self.mouse_button_up_event(MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Moved {
            self.mouse_motion_event(x, y);
        }
    }

    fn raw_mouse_motion(&mut self, _dx: f32, _dy: f32) {
        //
    }

    fn window_minimized_event(&mut self) {
        //
    }

    fn window_restored_event(&mut self) {
        //
    }

    fn quit_requested_event(&mut self) {
        //
    }

    fn files_dropped_event(&mut self) {
        //
    }
}

impl Window {
    fn handle_signals(&mut self) {
        //
    }

    fn pointer_update(&mut self) -> PointerUpdate {
        PointerUpdate {
            pointer: self.pointer_info(),
            current: self.pointer_state.clone(),
            coalesced: Vec::new(),
            predicted: Vec::new(),
        }
    }

    fn pointer_info(&self) -> PointerInfo {
        PointerInfo {
            pointer_id: None,
            persistent_device_id: None,
            pointer_type: PointerType::Mouse,
        }
    }

    fn get_image(&mut self) -> Image {
        todo!()
    }
}

fn convert_mouse_button(button: MouseButton) -> Option<ui_events::pointer::PointerButton> {
    use ui_events::pointer::PointerButton;
    match button {
        MouseButton::Left => Some(PointerButton::Primary),
        MouseButton::Right => Some(PointerButton::Secondary),
        MouseButton::Middle => Some(PointerButton::Auxiliary),
        MouseButton::Unknown => None,
    }
}

fn convert_keycode(state: KeyState, key: KeyCode, mods: KeyMods, repeat: bool) -> KeyboardEvent {
    // List partially generated with AI, I'm not confident about all of it.
    let (key, code) = match key {
        KeyCode::Space => (char_key(" "), Code::Space),
        KeyCode::Apostrophe => (char_key("'"), Code::Quote),
        KeyCode::Comma => (char_key(","), Code::Comma),
        KeyCode::Minus => (char_key("-"), Code::Minus),
        KeyCode::Period => (char_key("."), Code::Period),
        KeyCode::Slash => (char_key("/"), Code::Slash),
        KeyCode::Key0 => (char_key("0"), Code::Digit0),
        KeyCode::Key1 => (char_key("1"), Code::Digit1),
        KeyCode::Key2 => (char_key("2"), Code::Digit2),
        KeyCode::Key3 => (char_key("3"), Code::Digit3),
        KeyCode::Key4 => (char_key("4"), Code::Digit4),
        KeyCode::Key5 => (char_key("5"), Code::Digit5),
        KeyCode::Key6 => (char_key("6"), Code::Digit6),
        KeyCode::Key7 => (char_key("7"), Code::Digit7),
        KeyCode::Key8 => (char_key("8"), Code::Digit8),
        KeyCode::Key9 => (char_key("9"), Code::Digit9),
        KeyCode::Semicolon => (char_key(";"), Code::Semicolon),
        KeyCode::Equal => (char_key("="), Code::Equal),
        KeyCode::A => (char_key("a"), Code::KeyA),
        KeyCode::B => (char_key("b"), Code::KeyB),
        KeyCode::C => (char_key("c"), Code::KeyC),
        KeyCode::D => (char_key("d"), Code::KeyD),
        KeyCode::E => (char_key("e"), Code::KeyE),
        KeyCode::F => (char_key("f"), Code::KeyF),
        KeyCode::G => (char_key("g"), Code::KeyG),
        KeyCode::H => (char_key("h"), Code::KeyH),
        KeyCode::I => (char_key("i"), Code::KeyI),
        KeyCode::J => (char_key("j"), Code::KeyJ),
        KeyCode::K => (char_key("k"), Code::KeyK),
        KeyCode::L => (char_key("l"), Code::KeyL),
        KeyCode::M => (char_key("m"), Code::KeyM),
        KeyCode::N => (char_key("n"), Code::KeyN),
        KeyCode::O => (char_key("o"), Code::KeyO),
        KeyCode::P => (char_key("p"), Code::KeyP),
        KeyCode::Q => (char_key("q"), Code::KeyQ),
        KeyCode::R => (char_key("r"), Code::KeyR),
        KeyCode::S => (char_key("s"), Code::KeyS),
        KeyCode::T => (char_key("t"), Code::KeyT),
        KeyCode::U => (char_key("u"), Code::KeyU),
        KeyCode::V => (char_key("v"), Code::KeyV),
        KeyCode::W => (char_key("w"), Code::KeyW),
        KeyCode::X => (char_key("x"), Code::KeyX),
        KeyCode::Y => (char_key("y"), Code::KeyY),
        KeyCode::Z => (char_key("z"), Code::KeyZ),
        KeyCode::LeftBracket => (char_key("["), Code::BracketLeft),
        KeyCode::Backslash => (char_key("\\"), Code::Backslash),
        KeyCode::RightBracket => (char_key("]"), Code::BracketRight),
        KeyCode::GraveAccent => (char_key("`"), Code::Backquote),
        KeyCode::World1 => todo!(),
        KeyCode::World2 => todo!(),
        KeyCode::Escape => (NamedKey::Escape.into(), Code::Escape),
        KeyCode::Enter => (NamedKey::Enter.into(), Code::Enter),
        KeyCode::Tab => (NamedKey::Tab.into(), Code::Tab),
        KeyCode::Backspace => (NamedKey::Backspace.into(), Code::Backspace),
        KeyCode::Insert => (NamedKey::Insert.into(), Code::Insert),
        KeyCode::Delete => (NamedKey::Delete.into(), Code::Delete),
        KeyCode::Right => (NamedKey::ArrowRight.into(), Code::ArrowRight),
        KeyCode::Left => (NamedKey::ArrowLeft.into(), Code::ArrowLeft),
        KeyCode::Down => (NamedKey::ArrowDown.into(), Code::ArrowDown),
        KeyCode::Up => (NamedKey::ArrowUp.into(), Code::ArrowUp),
        KeyCode::PageUp => (NamedKey::PageUp.into(), Code::PageUp),
        KeyCode::PageDown => (NamedKey::PageDown.into(), Code::PageDown),
        KeyCode::Home => (NamedKey::Home.into(), Code::Home),
        KeyCode::End => (NamedKey::End.into(), Code::End),
        KeyCode::CapsLock => (NamedKey::CapsLock.into(), Code::CapsLock),
        KeyCode::ScrollLock => (NamedKey::ScrollLock.into(), Code::ScrollLock),
        KeyCode::NumLock => (NamedKey::NumLock.into(), Code::NumLock),
        KeyCode::PrintScreen => (NamedKey::PrintScreen.into(), Code::PrintScreen),
        KeyCode::Pause => (NamedKey::Pause.into(), Code::Pause),
        KeyCode::F1 => (NamedKey::F1.into(), Code::F1),
        KeyCode::F2 => (NamedKey::F2.into(), Code::F2),
        KeyCode::F3 => (NamedKey::F3.into(), Code::F3),
        KeyCode::F4 => (NamedKey::F4.into(), Code::F4),
        KeyCode::F5 => (NamedKey::F5.into(), Code::F5),
        KeyCode::F6 => (NamedKey::F6.into(), Code::F6),
        KeyCode::F7 => (NamedKey::F7.into(), Code::F7),
        KeyCode::F8 => (NamedKey::F8.into(), Code::F8),
        KeyCode::F9 => (NamedKey::F9.into(), Code::F9),
        KeyCode::F10 => (NamedKey::F10.into(), Code::F10),
        KeyCode::F11 => (NamedKey::F11.into(), Code::F11),
        KeyCode::F12 => (NamedKey::F12.into(), Code::F12),
        KeyCode::F13 => (NamedKey::F13.into(), Code::F13),
        KeyCode::F14 => (NamedKey::F14.into(), Code::F14),
        KeyCode::F15 => (NamedKey::F15.into(), Code::F15),
        KeyCode::F16 => (NamedKey::F16.into(), Code::F16),
        KeyCode::F17 => (NamedKey::F17.into(), Code::F17),
        KeyCode::F18 => (NamedKey::F18.into(), Code::F18),
        KeyCode::F19 => (NamedKey::F19.into(), Code::F19),
        KeyCode::F20 => (NamedKey::F20.into(), Code::F20),
        KeyCode::F21 => (NamedKey::F21.into(), Code::F21),
        KeyCode::F22 => (NamedKey::F22.into(), Code::F22),
        KeyCode::F23 => (NamedKey::F23.into(), Code::F23),
        KeyCode::F24 => (NamedKey::F24.into(), Code::F24),
        KeyCode::F25 => (NamedKey::F25.into(), Code::F25),
        KeyCode::Kp0 => (char_key("0"), Code::Numpad0),
        KeyCode::Kp1 => (char_key("1"), Code::Numpad1),
        KeyCode::Kp2 => (char_key("2"), Code::Numpad2),
        KeyCode::Kp3 => (char_key("3"), Code::Numpad3),
        KeyCode::Kp4 => (char_key("4"), Code::Numpad4),
        KeyCode::Kp5 => (char_key("5"), Code::Numpad5),
        KeyCode::Kp6 => (char_key("6"), Code::Numpad6),
        KeyCode::Kp7 => (char_key("7"), Code::Numpad7),
        KeyCode::Kp8 => (char_key("8"), Code::Numpad8),
        KeyCode::Kp9 => (char_key("9"), Code::Numpad9),
        KeyCode::KpDecimal => (char_key("."), Code::NumpadDecimal),
        KeyCode::KpDivide => (char_key("/"), Code::NumpadDivide),
        KeyCode::KpMultiply => (char_key("*"), Code::NumpadMultiply),
        KeyCode::KpSubtract => (char_key("-"), Code::NumpadSubtract),
        KeyCode::KpAdd => (char_key("+"), Code::NumpadAdd),
        KeyCode::KpEnter => (char_key("\n"), Code::NumpadEnter),
        KeyCode::KpEqual => (char_key("="), Code::NumpadEqual),
        KeyCode::LeftShift => (NamedKey::Shift.into(), Code::ShiftLeft),
        KeyCode::LeftControl => (NamedKey::Control.into(), Code::ControlLeft),
        KeyCode::LeftAlt => (NamedKey::Alt.into(), Code::AltLeft),
        KeyCode::LeftSuper => (NamedKey::Meta.into(), Code::MetaLeft),
        KeyCode::RightShift => (NamedKey::Shift.into(), Code::ShiftRight),
        KeyCode::RightControl => (NamedKey::Control.into(), Code::ControlRight),
        KeyCode::RightAlt => (NamedKey::Alt.into(), Code::AltRight),
        KeyCode::RightSuper => (NamedKey::Meta.into(), Code::MetaRight),
        KeyCode::Menu => (NamedKey::ContextMenu.into(), Code::ContextMenu),
        KeyCode::Back => (NamedKey::Backspace.into(), Code::Backspace),
        KeyCode::Unknown => (NamedKey::Unidentified.into(), Code::Unidentified),
    };

    let location = match code {
        Code::ShiftLeft | Code::ControlLeft | Code::AltLeft | Code::MetaLeft => Location::Left,
        Code::ShiftRight | Code::ControlRight | Code::AltRight | Code::MetaRight => Location::Right,
        _ => Location::Standard,
    };

    KeyboardEvent {
        state,
        key,
        code,
        location,
        modifiers: convert_keymods(mods),
        repeat,
        is_composing: false,
    }
}

fn convert_keymods(mods: KeyMods) -> Modifiers {
    let mut modifiers = Modifiers::empty();

    if mods.shift {
        modifiers.insert(Modifiers::SHIFT);
    }
    if mods.alt {
        modifiers.insert(Modifiers::ALT);
    }
    if mods.ctrl {
        modifiers.insert(Modifiers::CONTROL);
    }
    // TODO - I *think* logo is the Meta key?
    if mods.logo {
        modifiers.insert(Modifiers::META);
    }

    modifiers
}

fn char_key(chars: &'static str) -> Key {
    Key::Character(chars.to_string())
}

fn run_app() {
    //
}

/*

let mut ctx: Box<dyn miniquad::RenderingBackend> =
            miniquad::window::new_rendering_backend();

    pub fn from_rgba8(width: u16, height: u16, bytes: &[u8]) -> Texture2D {
        let texture = get_quad_context().new_texture_from_rgba8(width, height, bytes);
        let ctx = get_context();
        let texture = ctx.textures.store_texture(texture);
        let texture = Texture2D { texture };
        texture.set_filter(ctx.default_filter_mode);

        ctx.texture_batcher.add_unbatched(&texture);

        texture
    }


*/
