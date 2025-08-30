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