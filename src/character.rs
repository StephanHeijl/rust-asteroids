use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::f32;


pub trait Character {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_size(&self) -> f32;
    fn get_color(&self) -> Color;
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, x: f32);
    //fn draw(&self, canvas: &mut Canvas<Window>);

    fn draw(&self, canvas: &mut Canvas<Window>) {
        // Set draw color to character color
        canvas.set_draw_color(self.get_color());
        self.draw_character(canvas);
        // Return to background color
        canvas.set_draw_color(Color::RGB(0, 0, 0));
    }

    fn draw_character(&self, canvas : &mut Canvas<Window>);
    fn step(&mut self);

    fn wrap(&mut self) {
        let size = self.get_size();
        if self.get_x() > 640.0 + size {
            self.set_x(size * -1.0);
        } else if self.get_x() < size * -1.0 {
            self.set_x(640.0 + size);
        }
        if self.get_y() > 480.0 + size {
            self.set_y(size * -1.0);
        } else if self.get_y() < size * -1.0 {
            self.set_y(480.0 + size);
        }
    }

    fn check_out_of_stage(&self) -> bool {
        return self.get_x() > 640.0 + self.get_size() ||
               self.get_y() > 480.0 + self.get_size() ||
               self.get_x() < self.get_size() * -1.0 ||
               self.get_y() < self.get_size() * -1.0;
    }
}