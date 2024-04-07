use app::Application;

extern crate gl;
extern crate glfw;

mod app;

fn main() {
    let mut app = Application::new(1024, 720, "Mandlebrot");

    app.set_event_handler(|_event| {
        // TODO: handle stuff here in future
    });

    // Main loop
    while app.is_running() {
        // Handle events
        app.handle_events();

        // Render
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.1, 0.1, 0.3, 1.0);
        }

        // Swap buffers
        app.swap_window_buffers();
    }
}
