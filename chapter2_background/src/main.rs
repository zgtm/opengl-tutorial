use std::error::Error;
use std::ffi::{CStr, CString};
use std::ops::Deref;
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

        // if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
        //     println!("Running on {}", renderer.to_string_lossy());
        // }
        // if let Some(version) = get_gl_string(&gl, gl::VERSION) {
        //     println!("OpenGL Version {}", version.to_string_lossy());
        // }
        // if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
        //     println!("Shaders version on {}", shaders_version.to_string_lossy());
        // }

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

// impl Deref for Renderer {
//     type Target = gl::Gl;

//     fn deref(&self) -> &Self::Target {
//         &self.gl
//     }
// }

// fn get_gl_string(gl: &gl::Gl, variant: gl::types::GLenum) -> Option<&'static CStr> {
//     unsafe {
//         let s = gl.GetString(variant);
//         (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
//     }
// }

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
