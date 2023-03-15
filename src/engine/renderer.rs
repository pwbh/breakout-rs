use sdl2::{video::GLProfile, EventPump};

pub struct Renderer {
    timer: sdl2::TimerSubsystem,
    window: sdl2::video::Window,
    event_pump: EventPump,
    max_fps: u32,
    buffer_color: (u8, u8, u8),
}

impl Renderer {
    /// Creates the window, initializes OpenGL context, and start the renderer
    /// by default the created window will cap the max fps to 60
    pub fn build(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let sdl_context = match sdl2::init() {
            Ok(sdl) => sdl,
            Err(e) => return Err(e),
        };

        let video_subsystem = match sdl_context.video() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = match video_subsystem
            .window(title, width, height)
            .opengl()
            .resizable()
            .build()
        {
            Ok(w) => w,
            Err(_) => return Err(String::from("Couldn't create a window instance.")),
        };

        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (3, 3));

        let timer = sdl_context.timer()?;
        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            timer,
            window,
            event_pump,
            max_fps: 60,
            buffer_color: (0, 0, 0),
        })
    }

    pub fn window(&self) -> &sdl2::video::Window {
        &self.window
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.buffer_color = (r, g, b);
    }

    pub fn set_max_fps(&mut self, fps: u32) {
        if fps == 0 {
            panic!("FPS can't be negative")
        }

        self.max_fps = fps;
    }

    pub fn game_loop<T>(&mut self, l: &mut T)
    where
        T: FnMut(&sdl2::EventPump, f32),
    {
        self.set_gl_commands();

        let mut delta_time = 0f32;
        let mut last_frame = 0f32;

        'running: loop {
            let current_frame = self.timer.ticks() as f32;

            for event in self.event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'running,
                    _ => (),
                }
            }

            Renderer::clear_buffer(
                self.buffer_color.0,
                self.buffer_color.1,
                self.buffer_color.2,
            );

            l(&self.event_pump, delta_time);

            self.window.gl_swap_window();

            Renderer::limit_to_max_fps(self.max_fps);

            delta_time = current_frame - last_frame;
            last_frame = current_frame;
        }
    }

    // Only after the GL functions are initialized we can start doing preperatioins and etc
    fn set_gl_commands(&self) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    fn limit_to_max_fps(max_fps: u32) {
        std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / max_fps));
    }

    fn clear_buffer(r: u8, g: u8, b: u8) {
        unsafe {
            gl::ClearColor(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}
