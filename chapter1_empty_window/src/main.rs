use std::error::Error;
use glwindow::AppControl;
use glwindow::event::{WindowEvent, KeyEvent};
use glwindow::keyboard::{Key, NamedKey::Escape};

pub struct State {}
pub struct Renderer {}

impl glwindow::AppRenderer for Renderer {
    type AppState = State;
    fn new<D: glwindow::GlDisplay>(_gl_display: &D) -> Self {Renderer{}}
    fn draw(&self, _app_state: &mut State) {}
}

fn handle_event(_app_state: &mut State, event: WindowEvent)
                -> Result<AppControl, Box<dyn Error>> {
    let mut exit = false;
    match event {
        WindowEvent::CloseRequested => {
            exit = true;
        }
        WindowEvent::KeyboardInput{event: KeyEvent {logical_key: Key::Named(Escape),.. },..} => {
            exit = true;
        }
        _ => (),
    }

    Ok(if exit { AppControl::Exit } else { AppControl::Continue })
}

fn main() -> Result<(), Box<dyn Error>> {
    let app_state = State{};
    glwindow::Window::<_,_,Renderer>::new()
        .run(app_state, handle_event as glwindow::HandleFn<_>)
}
