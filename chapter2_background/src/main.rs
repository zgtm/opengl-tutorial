use std::error::Error;
use std::ffi::CString;
use glwindow::AppControl;
use glwindow::event::{WindowEvent, KeyEvent};
use glwindow::keyboard::{Key, NamedKey::Escape};

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
    pub use Gles2 as Gl;
}

pub struct State {}
pub struct Renderer {
    gl: gl::Gl,
}

impl glwindow::AppRenderer for Renderer {
    type AppState = State;

    fn new<D: glwindow::GlDisplay>(gl_display: &D) -> Self {
        let gl = gl::Gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });

        Self { gl }
    }

    fn draw(&self, _state: &mut State) {
        unsafe {
            self.gl.ClearColor(0.1, 0.1, 0.1, 0.9);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }
}

fn handle_event(_app_state: &mut State, event: WindowEvent)
                -> Result<AppControl, Box<dyn Error>> {
    let mut exit = false;
    match event {
        WindowEvent::CloseRequested => {
            exit = true;
        }
        WindowEvent::KeyboardInput { event: KeyEvent { logical_key: Key::Named(Escape), .. }, .. } => {
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
