extern crate gl;
extern crate glfw;

use std::ffi::c_void;

use app::Application;
use shader::Shader;

mod app;
mod shader;

fn main() {
    let mut app = Application::new(1024, 720, "Mandelbrot");

    app.set_event_handler(|_event| {
        // TODO: handle stuff here in future
    });

    let triangle_program = unsafe {
        match Shader::new("shaders/triangle_vs.glsl", "shaders/triangle_fs.glsl") {
            Ok(prog) => prog,
            Err(err) => panic!("Shader compilation failed:\n{}", err),
        }
    };

    #[rustfmt::skip]
    let verts: [f32; 6] = [
        0.0, 0.5,
        0.5, -0.5,
        -0.5, -0.5,
    ];

    let triangle_vao = unsafe {
        let (mut vao, mut vbo) = (0, 0);

        // Create vertex array
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create vertex buffer
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&verts) as isize,
            verts.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // Set vertex attributes
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<f32>() as i32 * 2,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        vao
    };

    // Main loop
    while app.is_running() {
        // Handle events
        app.handle_events();

        // Render
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);

            triangle_program.use_program();
            gl::BindVertexArray(triangle_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Swap buffers
        app.swap_window_buffers();
    }
}
