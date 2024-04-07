extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use std::ffi::c_void;

use app::Application;
use glfw::{Action, Key, WindowEvent};
use shader::Shader;

mod app;
mod shader;

fn main() {
    let (mut app, events) = Application::new(1024, 720, "Mandelbrot");

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

    let mut camera_pos = glm::vec2(0.0, 0.0);
    let mut zoom = 1.0;
    //
    // Main loop
    while app.is_window_open() {
        // Handle events
        app.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Close => app.close_window(),
                WindowEvent::FramebufferSize(w, h) => unsafe { gl::Viewport(0, 0, w, h) },
                WindowEvent::Key(Key::Left, _, Action::Press, _) => camera_pos.x -= 0.1,
                WindowEvent::Key(Key::Right, _, Action::Press, _) => camera_pos.x += 0.1,
                WindowEvent::Key(Key::Up, _, Action::Press, _) => camera_pos.y += 0.1,
                WindowEvent::Key(Key::Down, _, Action::Press, _) => camera_pos.y -= 0.1,
                WindowEvent::Key(Key::Minus, _, Action::Press, _) => zoom *= 0.8,
                WindowEvent::Key(Key::Equal, _, Action::Press, _) => zoom *= 1.25,
                _ => {}
            }
        }

        let mut view = glm::Mat4::identity();
        view = glm::translate(&view, &glm::vec2_to_vec3(&-camera_pos));
        view = glm::scale(&view, &glm::vec3(zoom, zoom, 1.0));

        // Uniforms
        unsafe {
            gl::UniformMatrix4fv(
                triangle_program.get_uniform_location("view"),
                1,
                gl::FALSE,
                glm::value_ptr(&view).as_ptr(),
            );
        }

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
