use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use std::f32;
use crate::character::Character;
use rand::Rng;


pub struct Asteroid {
    x: f32,
    y: f32,
    rotation: f32,
    speed_x: f32,
    speed_y: f32,
    size: f32,
    color: Color,
    shape: Vec<(f32, f32)>
}

impl Asteroid {

    pub fn new() -> Asteroid {
        Asteroid {
            x: 0.0,
            y : 0.0,
            rotation: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            size: 25.0,
            color: Color::RGB(255, 255, 255),
            shape: vec![]
        }
    }

    pub fn make_random_asteroid(&self) -> Vec<(f32, f32)>{
        let h = self.x;
        let k = self.y;
        let theta = self.rotation * -1.0;
        let size = self.get_size();
        let step_size = 0.25;
        let mut piece = 0.0;
        let mut shape : Vec<(f32, f32)> = Vec::new();
        let mut rng = rand::thread_rng();

        while piece < f32::consts::PI * 2.0 {
            let r : f32 = rng.gen_range(0.7, 1.0);

            let p = (
                (h + (r * size) * (theta + piece).sin()).ceil(),
                (k + (r * size) * (theta + piece).cos()).ceil(),
            );
            shape.push(p);

            piece += step_size;
        }

        return shape;
    }

    pub fn init(&mut self, x : f32, y : f32) {
        self.x = x;
        self.y = y;

        // Set asteroid speed.
        let mut rng = rand::thread_rng();
        self.speed_x = rng.gen_range(-1.5, 1.5);
        self.speed_y = rng.gen_range(-1.5, 1.5);

        self.shape = self.make_random_asteroid();
    }
}

impl Character for Asteroid {
    fn get_x(&self) -> f32 {
        return self.x;
    }

    fn get_y(&self) -> f32 {
        return self.y;
    }
    fn get_size(&self) -> f32 {
        return self.size;
    }
    fn get_color(&self) -> Color {
        return self.color;
    }
    fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    fn draw_character(&self, canvas : &mut Canvas<Window>) {
        let x = self.x;
        let y = self.y;

        if self.shape.len() > 0 {
            for p in 0..self.shape.len() - 1 {
                canvas.draw_line(
                    ((self.shape[p].0 + x) as i32, (self.shape[p].1 + y) as i32),
                    ((self.shape[p + 1].0 + x) as i32, (self.shape[p + 1].1 + y )as i32)
                ).expect("Could not draw line.");
            }
            let l = self.shape.len() - 1;
            canvas.draw_line(
                ((self.shape[0].0 + x) as i32, (self.shape[0].1 + y) as i32),
                ((self.shape[l].0 + x) as i32, (self.shape[l].1 + y) as i32)
            ).expect("Could not draw line.");
        }


    }

    fn step(&mut self) {
        // Space movement control
        self.set_x(self.x + self.speed_x);
        self.set_y(self.y + self.speed_y);

        if self.check_out_of_stage() {
            self.wrap()
        }
    }
}