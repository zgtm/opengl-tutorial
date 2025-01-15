use std::error::Error;
use std::ffi::{CStr, CString};
use std::time::Instant;
use glwindow::AppControl;
use glwindow::event::{WindowEvent, KeyEvent};
use glwindow::keyboard::{Key, NamedKey::Escape};

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
    pub use Gles2 as Gl;
}

pub struct State {
    begin: Instant,
}

pub struct Renderer {
    gl: gl::Gl,
    viewport_size: (i32, i32),
    program: gl::types::GLuint,
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    rotation: gl::types::GLint,
    perspective: gl::types::GLint,
}

static VERTEX_DATA: [f32; 18] = [
    -0.5, -0.5,  0.0,     0.8,  0.8,  0.0,
     0.0,  0.5,  0.0,     0.0,  0.8,  0.8,
     0.5, -0.5,  0.0,     0.8,  0.0,  0.8,
];

const VERTEX_SHADER_SOURCE: &CStr = c"
#version 410 core

uniform mat3 rotation;
uniform mat4 perspective;

const mat4 translation = mat4(1.0, 0.0, 0.0, 0.0,
                              0.0, 1.0, 0.0, 0.0,
                              0.0, 0.0, 1.0, 0.0,
                              0.0, 0.0, -1.5, 1.0);

in vec3 position;
in vec3 color;

out vec3 v_color;

void main() {
    gl_Position = perspective * translation * vec4(rotation * position, 1.0);
    v_color = color;
}
";

const FRAGMENT_SHADER_SOURCE: &CStr = c"
#version 410 core

out vec4 color;

in vec3 v_color;

void main() {
    color = vec4(v_color, 1.0);
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

            let pos_attrib =
                gl.GetAttribLocation(program, c"position".as_ptr() as *const _);
            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);

            let color_attrib =
                gl.GetAttribLocation(program, c"color".as_ptr() as *const _);
            gl.VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (3 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);

            let rotation =
                gl.GetUniformLocation(program, c"rotation".as_ptr() as *const _);
            let perspective =
                gl.GetUniformLocation(program, c"perspective".as_ptr() as *const _);

            Self { gl, viewport_size: (0, 0), program, vao, vbo, rotation, perspective }
        }
    }

    fn draw(&self, state: &mut State) {
        let time = Instant::now().duration_since(state.begin).as_millis() % 5000;
        let phi = (time as f32) / 5000.0 * 2.0 * std::f32::consts::PI;

        let rotation = glam::Mat3::from_rotation_y(phi);
        let aspect_ratio = self.viewport_size.0 as f32 / self.viewport_size.1 as f32;
        let perspective = glam::Mat4::perspective_rh_gl(1.0, aspect_ratio, 0.5, 10.0);

        unsafe {
            self.gl.UseProgram(self.program);
            self.gl.UniformMatrix3fv(self.rotation, 1, 0, (&rotation as *const _) as *const _);
            self.gl.UniformMatrix4fv(self.perspective, 1, 0, (&perspective as *const _) as *const _);

            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.gl.ClearColor(0.1, 0.1, 0.1, 0.9);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    fn resize(&mut self, width: i32, height: i32) {
        self.viewport_size = (width, height);
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
    let app_state = State{
        begin: Instant::now(),
    };
    glwindow::Window::<_,_,Renderer>::new()
        .run(app_state, handle_event as glwindow::HandleFn<_>)
}
