use speedy2d::color::Color;
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{WindowHandler, WindowHelper};

struct MyWindowHandler{}
impl WindowHandler for MyWindowHandler{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D){
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
    }
}
pub fn main() {
    let window = Window::new_centered("Test", (640, 480)).unwrap();
    window.run_loop(MyWindowHandler{});
}
