extern crate sdl2;
extern crate rand;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod character;
mod spaceship;
mod asteroid;
mod bullet;

use character::Character;
use spaceship::Spaceship;
use asteroid::Asteroid;
use std::collections::HashSet;


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("SDL2", 640, 480)
        .position_centered().build().map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .accelerated().build().map_err(|e| e.to_string())?;

    let mut timer = sdl_context.timer()?;

    let mut events = sdl_context.event_pump()?;

    let mut player = Spaceship::new();
    player.set_x(320.0);
    player.set_y(240.0);

    let mut enemy = Asteroid::new();
    enemy.init();
    enemy.set_x(200.0);
    enemy.set_y(200.0);

    let mut enemies = vec![enemy];

    let mut previous_ticks = 0;
    let mut tt_previous_ticks = 0;
    let mut ticks_taken;
    let update_rate = 144;
    let target_ms = 1000 / update_rate;
    let target_tick_rate = 1000 / 60;


    let mut running = true;
    while running {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                    player.up();
                },
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                    player.down();
                },
                _ => {}
            }
        }

        let pressed_keys : HashSet<Keycode> = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        // Rotating and firing need to happen simultaneously.
        if pressed_keys.contains(&Keycode::Left) {
            player.left();
        }
        if pressed_keys.contains(&Keycode::Right) {
            player.right();
        }
        if pressed_keys.contains(&Keycode::Space) {
            player.fire();
        }


        if (timer.ticks() - tt_previous_ticks) > target_tick_rate {
            canvas.clear();
            // copy the frame to the canvas
            player.step();
            player.draw(&mut canvas);

            for enemy in enemies.iter_mut() {
                enemy.step();
                enemy.draw(&mut canvas);
            }

            canvas.present();
            tt_previous_ticks = timer.ticks()
        }

        ticks_taken = timer.ticks() - previous_ticks;
        if ticks_taken < target_ms {
            std::thread::sleep(Duration::from_millis((target_ms - ticks_taken) as u64));
        }
        previous_ticks = timer.ticks();
    }

    Ok(())
}