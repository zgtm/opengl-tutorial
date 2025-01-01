use std::error::Error;
use std::ffi::{CStr, CString};
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
    program: gl::types::GLuint,
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    gl: gl::Gl,
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 9] = [
    -0.5, -0.5,  1.0,
     0.0,  0.5,  0.0,
     0.5, -0.5,  0.0,
];


const VERTEX_SHADER_SOURCE: &CStr = c"#version 410 core
in vec3 position;

void main() {
    gl_Position = vec4(position, 1.0f);
}
";

const FRAGMENT_SHADER_SOURCE: &CStr = c"#version 410 core

void main() {
    gl_FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
";

impl glwindow::AppRenderer for Renderer {
    type AppState = State;

    fn new<D: glwindow::GlDisplay>(gl_display: &D) -> Self {
        unsafe {
            let gl = gl::Gl::load_with(|symbol| {
                let symbol = CString::new(symbol).unwrap();
                gl_display.get_proc_address(symbol.as_c_str()).cast()
            });

            let vertex_shader = gl.CreateShader(gl::VERTEX_SHADER);
            gl.ShaderSource(vertex_shader, 1, [VERTEX_SHADER_SOURCE.as_ptr()].as_ptr(), std::ptr::null());
            gl.CompileShader(vertex_shader);

            let fragment_shader = gl.CreateShader(gl::FRAGMENT_SHADER);
            gl.ShaderSource(fragment_shader, 1, [FRAGMENT_SHADER_SOURCE.as_ptr()].as_ptr(), std::ptr::null());
            gl.CompileShader(fragment_shader);

            let program = gl.CreateProgram();

            gl.AttachShader(program, vertex_shader);
            gl.AttachShader(program, fragment_shader);

            gl.LinkProgram(program);

            gl.UseProgram(program);

            gl.DeleteShader(vertex_shader);
            gl.DeleteShader(fragment_shader);

            let mut vao = std::mem::zeroed();
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            let mut vbo = std::mem::zeroed();
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                VERTEX_DATA.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let pos_attrib = gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                3 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);

            Self { program, vao, vbo, gl }
        }
    }

    fn draw(&self, _state: &mut State) {
        unsafe {
            self.gl.UseProgram(self.program);

            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.gl.ClearColor(0.1, 0.1, 0.1, 0.9);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.program);
            self.gl.DeleteBuffers(1, &self.vbo);
            self.gl.DeleteVertexArrays(1, &self.vao);
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
