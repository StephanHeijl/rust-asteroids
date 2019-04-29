use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use std::f32;
use crate::character::Character;


fn is_inverse(a : f32, b : f32) -> bool {
    return a.signum() != b.signum();
}


pub struct Spaceship {
    x: f32,
    y: f32,
    rotation: f32,
    speed_x: f32,
    speed_y: f32,
    size: f32,
    color: Color
}

impl Spaceship {

    pub fn new() -> Spaceship {
        Spaceship {
            x: 0.0,
            y : 0.0,
            rotation: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            size: 25.0,
            color: Color::RGB(255, 255, 255)
        }
    }

    fn change_speed(&mut self, s : f32) {
        let max_speed = 3.0;
        let sx = s * self.rotation.cos();
        let sy = s * self.rotation.sin();
        if self.speed_x.abs() < max_speed || is_inverse(self.speed_x, sx){
            self.speed_x += sx;
        }
        if self.speed_y.abs() < max_speed || is_inverse(self.speed_y, sy){
            self.speed_y += sy;
        }
    }

    fn change_rotation(&mut self, r : f32) {
        self.rotation += r;
    }

    fn change_rotation_deg(&mut self, r : f32) {
        self.change_rotation((r / 180.0) * f32::consts::PI);
    }

    pub fn up(&mut self) {
        self.change_speed(1.0);
    }

    pub fn down(&mut self) {
        self.change_speed(-1.0);
    }

    pub fn left(&mut self) {
        self.change_rotation_deg(-10.0)
    }

    pub fn right(&mut self) {
        self.change_rotation_deg(10.0)
    }
}

impl Character for Spaceship {
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
        let start = (self.x.ceil() as i32, self.y.ceil() as i32);
        let r = self.get_size();

        let offset_l = f32::consts::PI * (0.15  - 0.5) as f32;
        let offset_r = f32::consts::PI * (-0.15 - 0.5)  as f32;

        let p1 = (
            (h + r * (theta + offset_l).sin()).ceil() as i32,
            (k + r * (theta + offset_l).cos()).ceil() as i32,
        );
        let p2 = (
            (h + r * (theta + offset_r).sin()).ceil() as i32,
            (k + r * (theta + offset_r).cos()).ceil() as i32,
        );

        canvas.draw_line(start, p1).expect("Could not draw line.");
        canvas.draw_line(p1, p2).expect("Could not draw line.");
        canvas.draw_line(p2, start).expect("Could not draw line.");
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