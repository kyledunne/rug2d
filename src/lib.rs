extern crate glutin;
extern crate gl;

static DEFAULT_WINDOW_SIZE: [u32; 2] = [1024, 768];

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let attributes = WindowAttributes::new()
            .with_title("Rug2d demo")
            .with_size(1440, 1000);
        let mut window = init_window_with_attributes(attributes);
        //initialization goes here

        //main loop
        let mut running = true;
        while running {
            let state = window.check_events();
            if state.close_requested {
                running = false;
            } else {
                if state.resize_requested {
                    window.resize(state.resize_size);
                }
            }
            window.render();
        }
        assert_eq!(2 + 2, 4);
    }
}

pub struct WindowAttributes {
    title: String,
    fullscreen: bool,
    size: [u32; 2],
} impl WindowAttributes {
    pub fn new() -> WindowAttributes {
        WindowAttributes {
            title: String::from("Rug2dWindow"),
            fullscreen: false,
            size: DEFAULT_WINDOW_SIZE,
        }
    }
    pub fn with_title(mut self, title: &str) -> WindowAttributes {
        self.title = String::from(title);
        self
    }
    pub fn with_fullscreen(mut self) -> WindowAttributes {
        self.fullscreen = true;
        self
    }
    pub fn with_size(mut self, width: u32, height: u32) -> WindowAttributes {
        self.size = [width, height];
        self
    }
}

pub struct WindowState {
    close_requested: bool,
    resize_requested: bool,
    resize_size: [u32; 2],
} impl WindowState {
    pub fn new() -> WindowState {
        WindowState {
            close_requested: false,
            resize_requested: false,
            resize_size: [0, 0],
        }
    }
}

pub struct Rug2dWindow {
    gl_window: glutin::GlWindow,
    events_loop: glutin::EventsLoop,
} impl Rug2dWindow {
    pub fn check_events(&mut self) -> WindowState {
        let mut window_state = WindowState::new();
        self.events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::Closed => {
                        window_state.close_requested = true;
                    },
                    glutin::WindowEvent::Resized(w, h) => {
                        window_state.resize_requested = true;
                        window_state.resize_size = [w, h];
                    },
                    _ => ()
                },
                _ => ()
            }
        });
        window_state
    }
    pub fn render(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        use glutin::GlContext;
        self.gl_window.swap_buffers().unwrap();
    }
    pub fn resize(&self, size: [u32; 2]) {
        use glutin::GlContext;
        self.gl_window.resize(size[0], size[1]);
    }
}

pub fn init_window_with_attributes(attributes: WindowAttributes) -> Rug2dWindow {
    use glutin::GlContext;
    let events_loop = glutin::EventsLoop::new();
    let mut window = glutin::WindowBuilder::new();
    if attributes.fullscreen {
        window = window.with_fullscreen(None);
    } else {
        window = window.with_title(attributes.title)
                       .with_dimensions(attributes.size[0], attributes.size[1]);
    }
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    Rug2dWindow {
        gl_window,
        events_loop,
    }
}

pub fn info() {
    glutin::EventsLoop::new();
    println!("rug2d: test version sucessfully executed.")
}

pub fn test_fn() {

}
