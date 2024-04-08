extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use glfw::{Action, Key, WindowEvent};
use std::{ffi::c_void, time::Instant};

use app::Application;
use shader::Shader;

mod app;
mod shader;

#[rustfmt::skip]
const VERTICES: [f32; 12] = [
     1.0,  1.0,
    -1.0,  1.0,
    -1.0, -1.0,
    -1.0, -1.0,
     1.0, -1.0,
     1.0,  1.0,
];

const PAN_SPEED: f32 = 0.5;
const ZOOM_SPEED: f32 = 1.25;

fn main() {
    let (mut app, events) = Application::new(1024, 720, "Mandelbrot");

    let mandelbrot_program = unsafe {
        match Shader::new("shaders/mandelbrot_vs.glsl", "shaders/mandelbrot_fs.glsl") {
            Ok(prog) => prog,
            Err(err) => panic!("Shader compilation failed:\n{}", err),
        }
    };

    let mandelbrot_vao = unsafe {
        let (mut vao, mut vbo) = (0, 0);

        // Create vertex array
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create vertex buffer
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr() as *const c_void,
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

        // Unbind objects
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        vao
    };

    let mut pan = glm::vec2(0.0, 0.0);
    let mut zoom = 1.0;

    let mut last_frame = Instant::now();

    // Main loop
    while app.is_window_open() {
        // Calculate time delta
        let now = Instant::now();
        let dt = (now - last_frame).as_secs_f32();
        last_frame = now;

        // Handle events
        app.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Close => app.close_window(),
                WindowEvent::FramebufferSize(w, h) => unsafe { gl::Viewport(0, 0, w, h) },
                _ => {}
            }
        }

        if [app.get_key(Key::Left), app.get_key(Key::A)].contains(&Action::Press) {
            pan.x -= PAN_SPEED / zoom * dt;
        } else if [app.get_key(Key::Right), app.get_key(Key::D)].contains(&Action::Press) {
            pan.x += PAN_SPEED / zoom * dt;
        }
        if [app.get_key(Key::Up), app.get_key(Key::W)].contains(&Action::Press) {
            pan.y += PAN_SPEED / zoom * dt;
        } else if [app.get_key(Key::Down), app.get_key(Key::S)].contains(&Action::Press) {
            pan.y -= PAN_SPEED / zoom * dt;
        }
        if app.get_key(Key::Minus) == Action::Press {
            zoom -= zoom * ZOOM_SPEED * dt;
        } else if app.get_key(Key::Equal) == Action::Press {
            zoom += zoom * ZOOM_SPEED * dt;
        }

        // Uniforms
        unsafe {
            mandelbrot_program.use_program();
            gl::Uniform2f(mandelbrot_program.get_uniform_location("pan"), pan.x, pan.y);
            gl::Uniform1f(mandelbrot_program.get_uniform_location("zoom"), zoom);
        }

        // Render
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);

            mandelbrot_program.use_program();
            gl::BindVertexArray(mandelbrot_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        // Swap buffers
        app.swap_window_buffers();
    }
}
