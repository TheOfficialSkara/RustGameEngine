extern crate glfw;
extern crate gl;

mod shader;

use shader::Shader;
use self::glfw::{Context, Key, Action};
use self::gl::types::*;
use std::sync::mpsc::Receiver;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;

const CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 800;

fn main() {
    let mut t = 0.0;

    // glfw: initialize and configure
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "Game Engine", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shader, vao, ebo) = unsafe {
        // build and compile our shaders
        let test_shader = Shader::new("src/testingShader.vs", "src/testingShader.fs");

        let vertices: [f32; 12] = [
             0.5,  0.5, 0.0, // top-right
             0.5, -0.5, 0.0, // bottom-right
            -0.5, -0.5, 0.0, // bottom-left
            -0.5,  0.5, 0.0, // top-left
        ];

        let indices: [u32; 6] = [
            0, 1, 3,
            1, 2, 3
        ];

        // Bind VAO
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Bind VBO
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Set vertex data
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        // Bind vbo
        let mut ebo = 0;
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        // Set vertex indices
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLint>()) as GLsizeiptr,
                       &indices[0] as *const u32 as *const c_void,
                       gl::STATIC_DRAW);

        // Unbind everything
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (test_shader, vao, ebo)
    };

    // render loop
    while !window.should_close() {
        // events
        process_events(&mut window, &events);

        // render
        unsafe {
            gl::ClearColor(CLEAR_COLOR[0], CLEAR_COLOR[1], CLEAR_COLOR[2], CLEAR_COLOR[3]);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw our first rectangle
            shader.use_program();
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            // Set uniform variables
            let c_str = CString::new("z".as_bytes()).unwrap();
            shader.set_float(&c_str, f32::sin(t) * 0.5 + 0.5);

            // Draw
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.swap_buffers();
        glfw.poll_events();
        t += 0.005;
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => { println!("{:?}", event); }
        }
    }
}
