use glfw::{Context, OpenGlProfileHint, WindowHint, WindowMode};

extern crate gl;
extern crate glfw;

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialize GLFW");
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    // Create window
    let (mut window, events) = glfw
        .create_window(1024, 576, "Mandlebrot", WindowMode::Windowed)
        .expect("Failed to create window");
    window.make_current();

    // Load OpenGL functions
    gl::load_with(|sym| window.get_proc_address(sym));

    // Main loop
    while !window.should_close() {
        // Poll events
        glfw.poll_events();

        // Render
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        // Swap buffers
        window.swap_buffers();
    }
}
