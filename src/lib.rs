extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

static DEFAULT_WINDOW_SIZE: [u32; 2] = [1024, 768];
static mut GRAPHICS_INITIALIZED: bool = false;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        test_fn();
        assert_eq!(2 + 2, 4);
    }
}

pub fn info() {
    println!("rug2d: test version sucessfully executed.")
}

pub fn test_fn() {
    let mut window = init_window("Sup", 1000, 500);
    'running: loop {
        window.render();
        if window.check_events() {
            break 'running;
        }
        //TODO set up a sync() method that considers the nanos since the last call to sync
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn init_window(title: &str, w: u32, h: u32) -> Rug2dWindow {
    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window: sdl2::video::Window = video_subsystem.window(title, w, h)
        .opengl()
        .build()
        .unwrap();

    window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    let event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap();

    //'running: loop {
    //::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));

    Rug2dWindow {
        window,
        event_pump,
    }
}

pub struct Rug2dWindow {
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
} impl Rug2dWindow {
    pub fn render(&self) {
        unsafe {
            //TODO figure out why this color isn't appearing
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.window.gl_swap_window();
    }
    pub fn check_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    println!("wow!");
                    return true;
                },
                _ => {}
            }
        }
        false
    }
}