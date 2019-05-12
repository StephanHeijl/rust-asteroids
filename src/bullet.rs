use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use std::f32;
use crate::character::Character;
//use rand::Rng;

#[derive(Debug)]
pub struct Bullet {
    x: f32,
    y: f32,
    rotation: f32,
    speed_x: f32,
    speed_y: f32,
    size: f32,
    color: Color,
    shape: Vec<(f32, f32)>,
    pub is_destroyed: bool,
}


impl Bullet {

    pub fn new() -> Bullet {
        Bullet {
            x: 0.0,
            y : 0.0,
            rotation: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            size: 5.0,
            color: Color::RGB(255, 255, 255),
            shape: vec![],
            is_destroyed: false,
        }
    }

    pub fn set_speed(&mut self, sx : f32, sy : f32) {
        self.speed_x = sx;
        self.speed_y = sy;
    }

    pub fn set_rotation(&mut self, rot : f32) {
        self.rotation = rot;
    }
}

impl Character for Bullet {
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
        let h = self.x;
        let k = self.y;
        let theta = self.rotation * -1.0;

        let r = self.get_size();
        let ninety_deg = f32::consts::PI * 0.5;

        let p1 = (
            (h + r * (theta + ninety_deg).sin()).ceil() as i32,
            (k + r * (theta + ninety_deg).cos()).ceil() as i32,
        );

        let start = (self.x.ceil() as i32, self.y.ceil() as i32);
        canvas.draw_line(start, p1).expect("Could not draw line.");
    }


    fn step(&mut self) {
        // Space movement control
        self.set_x(self.x + self.speed_x);
        self.set_y(self.y + self.speed_y);

        if self.check_out_of_stage() {
            self.destroy();
        }
    }

    fn destroy(&mut self) {
        self.is_destroyed = true;
    }

    fn get_center(&self) -> (i32, i32) {
        let theta = self.rotation * -1.0;
        let r = self.get_size();
        (
            (self.x + (r / 2.0) * (theta).sin()).ceil() as i32,
            (self.y + (r / 2.0) * (theta).cos()).ceil() as i32,
        )
    }
}