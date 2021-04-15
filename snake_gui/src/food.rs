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

use crate::draw::draw;
use crate::screen::{Position, Screen};
use crate::snake::Snake;
use piston_window::{types::Color, Context, G2d};
use rand::{thread_rng, Rng};

const FOOD_COLOR: Color = [0.7, 0.7, 0.7, 0.7];

#[derive(Debug, PartialEq)]
pub struct Food {
    pub position: Position,
}

impl Food {
    pub fn new() -> Food {
        Food {
            position: Position {
                x: thread_rng().gen_range(1..Screen::WIDTH),
                y: thread_rng().gen_range(1..Screen::HEIGHT),
            },
        }
    }

    pub fn spawn(&mut self, snake: &Snake) {
        while snake.tail.contains(&self.position) {
            self.position = Position {
                x: thread_rng().gen_range(1..Screen::WIDTH),
                y: thread_rng().gen_range(1..Screen::HEIGHT),
            };
        }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        draw(FOOD_COLOR, self.position.x, self.position.y, 1, 1, c, g);
    }
}
