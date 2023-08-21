extern crate sdl2;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Rect, Point},
    video::Window,
    render::{Texture, Canvas},
    keyboard::{Scancode, KeyboardState},
    image::LoadTexture,
};

use std::path::Path;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl() .build() .map_err(|e| e.to_string())?; let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let texture_creator = canvas.texture_creator();

    let textures = texture_creator.load_texture(Path::new("textures.png"))?;

    let mut player = Player::new(Rect::new(128, 128, 32, 32), 
                                 Rect::new(0, 0, 64, 64),
                                 &textures);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        player.move_player(event_pump.keyboard_state());

        canvas.set_draw_color(Color::RGB(45, 45, 45));
        canvas.clear();

        player.draw(&mut canvas)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}

struct Player<'a> {
    location: Rect,
    yaw: f32,
    texture_location: Rect,
    texture: &'a Texture<'a>,
}

impl Player<'_> {
    pub fn new<'a>(location: Rect, texture_location: Rect, texture: &'a Texture<'a>) -> Player {
        Player {
            location,
            yaw: 0.,
            texture_location,
            texture,
        }
    }

    pub fn move_player(&mut self, event: KeyboardState) {
        if event.is_scancode_pressed(Scancode::Up) {
            //self.location.y -= 16
        }
        if event.is_scancode_pressed(Scancode::Down) {
            //self.location.y += 16
        }
        if event.is_scancode_pressed(Scancode::Right) {
            self.yaw += 0.1;
            if self.yaw > std::f32::consts::PI * 2. {
                self.yaw = 0.;
            }
        }
        if event.is_scancode_pressed(Scancode::Left) {
            self.yaw -= 0.1;
            if self.yaw < 0. {
                self.yaw = std::f32::consts::PI * 2.;
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        // draw circle in the center
        canvas.copy(self.texture, self.texture_location, self.location)?;

        // same color as circle sprite
        canvas.set_draw_color(Color::RGB(99, 155, 255));

        // rotate 64(x) around center
        // rx = x * cos(a) - y * sin(a)
        let rx = (64. * self.yaw.cos() - 64. * self.yaw.sin()) as i32;
        // rotate 64(y) around center
        // ry = y * cos(a) + x * sin(a)
        let ry = (64. * self.yaw.cos() + 64. * self.yaw.sin()) as i32;

        //draw line in right direction
        canvas.draw_line(Point::new(self.location.x, self.location.y),
                         Point::new(rx + self.location.x, ry + self.location.y))?;

        // ok
        Ok(())
    }
}
