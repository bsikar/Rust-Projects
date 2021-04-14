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
use crate::snake::*;
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

    pub fn key_press(&mut self, key: Key) {
        match key {
            Key::Right | Key::D => self.snake.mv(Direction::Right),
            Key::Left | Key::A => self.snake.mv(Direction::Left),
            Key::Down | Key::S => self.snake.mv(Direction::Down),
            Key::Up | Key::W => self.snake.mv(Direction::Up),
            Key::Q => self.snake.mv(Direction::Still),
            _ => self.snake.mv(self.snake.direction),
        }
    }

    pub fn run(&mut self, key: Key, c: &Context, g: &mut G2d) {
        self.key_press(key);
        self.draw(c, g);
    }

    pub fn draw(&mut self, c: &Context, g: &mut G2d) {
        self.snake.draw(c, g);
        self.food.draw(c, g);
        if self.snake.position == self.food.position {
            self.snake.eat();
            self.food.spawn(&self.snake);
        }
    }

    pub fn over(&self) -> bool {
        !self.snake.is_alive()
    }
}
