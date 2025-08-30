extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use crate::piston::EventLoop;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, ButtonArgs, ButtonEvent, Button, ButtonState, Key};
use piston::window::WindowSettings;
use rand::Rng;

enum Direction{
    Up,Down,Left,Right
}

pub struct Segment{
    x: i32,
    y: i32
}

pub struct App {
    gl: GlGraphics, 
    segments: Vec<Segment>,
    direction: Direction,
    applex: i32,
    appley: i32,
    score: u32,
    gameover: bool,

}

impl App{
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let mut square_segments: Vec<[f64; 4]> = Vec::new();
        for i in &self.segments{
            let x = i.x as f64;
            let y = i.y as f64;
            square_segments.push(rectangle::square(x, y, 10.0));
        }

        let apple = rectangle::square(self.applex as f64, self.appley as f64, 10.0);

        self.gl.draw(args.viewport(), |c, gl|{
            clear(WHITE, gl);
            let transform = c.transform.trans(0.0,0.0).rot_deg(0.0);
            for i in square_segments {
                rectangle(BLUE, i, transform, gl);
            }
            rectangle(GREEN, apple, transform, gl);

        });

    }