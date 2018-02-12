extern crate sdl2;
extern crate gl;
extern crate glm;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use sdl2::event::WindowEvent;

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;

static FPS: u32 = 60;
static WINDOW_TARGET_SIZE: [u32; 2] = [1024, 768];

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(super::test_fn());
    }
}

pub fn test_fn() -> bool {
    init_window("Rug2d Test Window", WINDOW_TARGET_SIZE[0], WINDOW_TARGET_SIZE[1]);
    true
}

pub fn init_window(title: &str, width: u32, height: u32) {
    let sdl_context : sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let mut window : sdl2::video::Window = video_subsystem.window(title, width, height)
        .opengl()
        .build()
        .unwrap();
    window.set_minimum_size(500, 500);

    let ctx : sdl2::video::GLContext = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    let mut event_pump = sdl_context.event_pump().unwrap();


    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    let half_width = width as f32 / 2.0;
    let half_height = height as f32 / 2.0;
    let scaling_mat = glm::mat4(1.0 / (width as f32 / 2.0), 0.0, 0.0, 0.0,
                                                     0.0, 1.0 / (height as f32 / 2.0), 0.0, 0.0,
                                                     0.0, 0.0, 1.0, 0.0,
                                                     0.0, 0.0, 0.0, 1.0);

    unsafe {
        let cstring = std::ffi::CString::new(String::from("scalingMat")).unwrap();
        let scaling_mat_id = gl::GetUniformLocation(program, cstring.as_ptr());
        //TODO finish doing this (w/ http://www.opengl-tutorial.org/beginners-tutorials/tutorial-3-matrices/#scaling-matrices and
        //https://docs.rs/gl/0.10.0/gl/fn.UniformMatrix4fv.html)
        gl::ProgramUniformMatrix4fv(program, scaling_mat_id, 1, gl::FALSE, &scaling_mat[0][0]);


        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            2,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            0,
            ptr::null(),
        );
    }


    'running: loop {
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::Window { win_event: WindowEvent::SizeChanged(x, y), .. } => {
                    println!("window size changed")
                },
                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    // Cleanup
    unsafe {
        gl::DeleteProgram(program);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}

// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.0, 200.0, 0.0, 200.0, 200.0];

// Shader sources
static VS_SRC: &'static str = "
#version 150
in vec2 position;
uniform mat4 scalingMat;
void main() {
    gl_Position = scalingMat * vec4(position, 0.0, 1.0) - vec4(1.0, 1.0, 0.0, 0.0);
}";

static FS_SRC: &'static str = "
#version 150
out vec4 out_color;
void main() {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
}";

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}
