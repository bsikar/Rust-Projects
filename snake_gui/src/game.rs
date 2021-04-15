/* MIT License
 *
 * Copyright (c) 2021 Brighton Sikarskie
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use crate::food::Food;
use crate::screen::Screen;
use crate::snake::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::*;

pub struct Game {
    snake: Snake,
    food: Food,
}

impl Game {
    pub fn new(snake: Snake, food: Food) -> Game {
        Game {
            snake: snake,
            food: food,
        }
    }

    pub fn update(&mut self, args: &UpdateArgs, key: Key) {
        self.snake.update(args.dt, self.key_direction(key));
        if self.snake.position == self.food.position {
            self.snake.eat();
            self.food.spawn(&self.snake);
        }
    }

    fn key_direction(&self, key: Key) -> Direction {
        return {
            match key {
                Key::Right | Key::D => Direction::Right,
                Key::Left | Key::A => Direction::Left,
                Key::Down | Key::S => Direction::Down,
                Key::Up | Key::W => Direction::Up,
                Key::Q => Direction::Still,
                _ => self.snake.direction,
            }
        };
    }

    pub fn draw(&mut self, c: &Context, g: &mut G2d) {
        self.snake.draw(c, g);
        self.food.draw(c, g);
    }

    pub fn over(&self) -> bool {
        !self.snake.is_alive()
    }

    pub fn draw_game_over(&self, args: RenderArgs) {
        let mut gl = GlGraphics::new(OpenGL::V3_2);
        let mut glyphs =
            GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();
        gl.draw(args.viewport(), |c, g| {
            clear([0.5; 4], g);
            text(
                [1.0, 0.99, 0.22, 1.0],
                30,
                format!("Final Length: {}", self.snake.length).as_str(),
                &mut glyphs,
                c.transform.trans(
                    f64::from(Screen::WIDTH * Screen::WIDTH),
                    f64::from(Screen::HEIGHT * Screen::HEIGHT),
                ),
                g,
            )
        })
        .expect("Failed to make end screen");
    }
}
