extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

static FPS: u32 = 60;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::test_fn();
        assert_eq!(2 + 2, 4);
    }
}

pub fn info() {
    println!("Rug2d version 0.1.0");
}

pub fn test_fn() {
    let mut window = init_window("Yoooooo...", 1024, 768);
    'running: loop {
        window.render();
        if window.check_events() {
            break 'running;
        }
        wait_until_next_frame(FPS);
    }
}

pub fn wait_until_next_frame(fps: u32) {
    //TODO set up a sync() method that considers the nanos since the last call to sync
    ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / fps));
}

pub fn init_window(title: &str, w: u32, h: u32) -> Rug2dWindow {
    fn find_sdl_gl_driver() -> Option<u32> {
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                return Some(index as u32);
            }
        }
        None
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(title, w, h)
        .opengl()
        .build()
        .unwrap();
    let mut canvas: sdl2::render::WindowCanvas = window.into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    canvas.window().gl_set_context_to_current();

    unsafe {
        gl::ClearColor(0.6, 0.0, 0.8, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();

    //'running: loop {
    //::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));

    Rug2dWindow {
        canvas,
        event_pump,
    }
}

pub struct Rug2dWindow {
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,
} impl Rug2dWindow {
    pub fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.canvas.present();
    }
    pub fn check_events(&mut self) -> bool {
        let mut should_quit = false;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    println!("Client close requested (rug2d::Rug2dWindow::check_events())");
                    should_quit = true;
                },
                _ => {}
            }
        }
        should_quit
    }
}