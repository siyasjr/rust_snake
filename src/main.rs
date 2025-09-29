extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::{thread_rng, Rng};
use std::collections::LinkedList;

const BLOCK_SIZE: f64 = 20.0;
const WIDTH: u32 = 20;
const HEIGHT: u32 = 15;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Snake {
    body: LinkedList<[u32; 2]>,
    dir: Direction,
    grow: bool,
}

impl Snake {
    fn new() -> Snake {
        let mut body = LinkedList::new();
        body.push_back([WIDTH / 2, HEIGHT / 2]);
        Snake {
            body,
            dir: Direction::Right,
            grow: false,
        }
    }

    fn head_position(&self) -> [u32; 2] {
        *self.body.front().expect("Snake has no body")
    }

    fn move_forward(&mut self) {
        let mut new_head = self.head_position();
        match self.dir {
            Direction::Up => {
                if new_head[1] == 0 {
                    new_head[1] = HEIGHT - 1;
                } else {
                    new_head[1] -= 1;
                }
            }
            Direction::Down => {
                new_head[1] = (new_head[1] + 1) % HEIGHT;
            }
            Direction::Left => {
                if new_head[0] == 0 {
                    new_head[0] = WIDTH - 1;
                } else {
                    new_head[0] -= 1;
                }
            }
            Direction::Right => {
                new_head[0] = (new_head[0] + 1) % WIDTH;
            }
        }
        self.body.push_front(new_head);
        if !self.grow {
            self.body.pop_back();
        } else {
            self.grow = false;
        }
    }

    fn change_direction(&mut self, dir: Direction) {
        if dir == Direction::Up && self.dir != Direction::Down
            || dir == Direction::Down && self.dir != Direction::Up
            || dir == Direction::Left && self.dir != Direction::Right
            || dir == Direction::Right && self.dir != Direction::Left
        {
            self.dir = dir;
        }
    }

    fn grow(&mut self) {
        self.grow = true;
    }

    fn check_self_collision(&self) -> bool {
        let head = self.head_position();
        self.body.iter().skip(1).any(|&pos| pos == head)
    }
}

struct Game {
    snake: Snake,
    food_pos: [u32; 2],
    game_over: bool,
}

impl Game {
    fn new() -> Game {
        let mut game = Game {
            snake: Snake::new(),
            food_pos: [0, 0],
            game_over: false,
        };
        game.place_food();
        game
    }

    fn place_food(&mut self) {
        let mut rng = thread_rng();
        loop {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            if !self.snake.body.contains(&[x, y]) {
                self.food_pos = [x, y];
                break;
            }
        }
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }
        self.snake.move_forward();
        let head = self.snake.head_position();
        // Check collision with self
        if self.snake.check_self_collision() {
            self.game_over = true;
        }
        // Check food
        if head == self.food_pos {
            self.snake.grow();
            self.place_food();
        }
    }
}

fn draw_block(color: [f32; 4], pos: [u32; 2], context: Context, g: &mut G2d) {
    let x = (pos[0] as f64) * BLOCK_SIZE;
    let y = (pos[1] as f64) * BLOCK_SIZE;
    rectangle(color, [x, y, BLOCK_SIZE, BLOCK_SIZE], context.transform, g);
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [WIDTH as f64 * BLOCK_SIZE, HEIGHT as f64 * BLOCK_SIZE])
            .exit_on_esc(true)
            .build()
            .expect("Failed to build PistonWindow");

    let mut game = Game::new();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => game.snake.change_direction(Direction::Up),
                Key::Down => game.snake.change_direction(Direction::Down),
                Key::Left => game.snake.change_direction(Direction::Left),
                Key::Right => game.snake.change_direction(Direction::Right),
                _ => {}
            }
        }

        if let Some(UpdateArgs { .. }) = event.update_args() {
            game.update();
        }

        window.draw_2d(&event, |context, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            // Draw snake
            for block in &game.snake.body {
                draw_block([0.0, 1.0, 0.0, 1.0], *block, context, g);
            }

            // Draw food
            draw_block([1.0, 0.0, 0.0, 1.0], game.food_pos, context, g);

            // Game Over text (simple)
            if game.game_over {
                // Could add more sophisticated text rendering with another crate if desired
            }
        });
    }
}
