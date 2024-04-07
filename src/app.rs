use glfw::{
    Context, Glfw, GlfwReceiver, OpenGlProfileHint, PWindow, WindowEvent, WindowHint, WindowMode,
};

pub struct Application {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    event_handler: Option<fn(WindowEvent)>,
}

impl Application {
    pub fn new(window_width: u32, window_height: u32, title: &str) -> Self {
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

        Self {
            glfw,
            window,
            events,
            event_handler: None,
        }
    }

    pub fn is_running(&self) -> bool {
        !self.window.should_close()
    }

    pub fn set_event_handler(&mut self, handler: fn(WindowEvent)) {
        self.event_handler = Some(handler);
    }

    pub fn handle_events(&mut self) {
        // Poll events
        self.glfw.poll_events();

        // Handle events
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Close => self.window.set_should_close(true),
                WindowEvent::FramebufferSize(w, h) => unsafe { gl::Viewport(0, 0, w, h) },
                _ => {
                    if let Some(handler) = self.event_handler {
                        handler(event);
                    }
                }
            }
        }
    }

    pub fn swap_window_buffers(&mut self) {
        self.window.swap_buffers();
    }
}
