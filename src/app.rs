extern crate gl;
extern crate glfw;

use glfw::{
    Context, Glfw, GlfwReceiver, OpenGlProfileHint, PWindow, WindowEvent, WindowHint, WindowMode,
};

pub struct Application {
    glfw: Glfw,
    window: PWindow,
}

impl Application {
    pub fn new(
        window_width: u32,
        window_height: u32,
        title: &str,
    ) -> (Self, GlfwReceiver<(f64, WindowEvent)>) {
        // Initialize GLFW
        let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialize GLFW");
        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

        // Create window
        let (mut window, events) = glfw
            .create_window(window_width, window_height, title, WindowMode::Windowed)
            .expect("Failed to create window");
        window.make_current();
        window.set_all_polling(true);

        // Load OpenGL functions
        gl::load_with(|sym| window.get_proc_address(sym));

        (Self { glfw, window }, events)
    }

    pub fn is_window_open(&self) -> bool {
        !self.window.should_close()
    }

    pub fn close_window(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    pub fn swap_window_buffers(&mut self) {
        self.window.swap_buffers();
    }
}
