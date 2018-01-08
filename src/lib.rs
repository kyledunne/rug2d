extern crate glutin;
extern crate gl;

#[cfg(test)]
mod tests {
    extern crate glutin;
    extern crate gl;

    #[test]
    fn it_works() {
        glutin::EventsLoop::new();
        println!("rug2d: test version sucessfully executed (from tests::it_works()).");
        assert_eq!(2 + 2, 4);
    }
}

pub fn info() {
    glutin::EventsLoop::new();
    println!("rug2d: test version sucessfully executed.")
}
