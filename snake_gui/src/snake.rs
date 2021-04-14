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
use piston_window::{types::Color, Context, G2d};

const SNAKE_COLOR: Color = [1., 1., 1., 1.];

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Still,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Snake {
    pub position: Position,
    length: u32,
    pub direction: Direction,
    tail: Vec<Position>,
    is_alive: bool,
}

impl Snake {
    pub fn new(x: u32, y: u32) -> Snake {
        Snake {
            position: Position { x: x, y: y },
            length: 1,
            direction: Direction::Still,
            tail: vec![Position { x: x, y: y }],
            is_alive: true,
        }
    }

    fn push(&mut self, position: Position) {
        self.tail.push(position);
    }

    fn pop(&mut self) {
        self.tail.pop();
    }

    fn is_valid(&self) -> bool {
        let x = self.position.x;
        let y = self.position.y;

        x > 0 && y > 0 && x < Screen::WIDTH && y < Screen::HEIGHT
    }

    pub fn mv(&mut self, direction: Direction) {
        if !self.is_valid() {
            self.is_alive = false;
            return;
        }

        match self.direction {
            Direction::Left => {
                if direction != Direction::Right {
                    self.direction = direction;
                }
            }
            Direction::Right => {
                if direction != Direction::Left {
                    self.direction = direction;
                }
            }
            Direction::Up => {
                if direction != Direction::Down {
                    self.direction = direction;
                }
            }
            Direction::Down => {
                if direction != Direction::Up {
                    self.direction = direction;
                }
            }
            Direction::Still => self.direction = direction,
        }

        // Note: I am using 2 match cases here for visibilty (I could have put this in the one up above).
        match self.direction {
            Direction::Left => {
                if self.position.x != 0 {
                    self.position.x -= 1;
                    self.pop();
                    self.push(self.position);
                }
            }
            Direction::Right => {
                if self.position.x != Screen::WIDTH {
                    self.position.x += 1;
                    self.pop();
                    self.push(self.position);
                }
            }
            Direction::Up => {
                if self.position.y != 0 {
                    self.position.y -= 1;
                    self.pop();
                    self.push(self.position);
                }
            }
            Direction::Down => {
                if self.position.y != Screen::HEIGHT {
                    self.position.y += 1;
                    self.pop();
                    self.push(self.position);
                }
            }
            Direction::Still => {}
        }
    }

    pub fn eat(&mut self) {
        match self.direction {
            Direction::Left => {
                self.push(Position {
                    x: self.position.x + 1,
                    y: self.position.y,
                });
            }
            Direction::Right => {
                self.push(Position {
                    x: self.position.x - 1,
                    y: self.position.y,
                });
            }
            Direction::Up => {
                self.push(Position {
                    x: self.position.x,
                    y: self.position.y - 1,
                });
            }
            Direction::Down => {
                self.push(Position {
                    x: self.position.x,
                    y: self.position.y + 1,
                });
            }
            Direction::Still => {}
        }
        self.length += 1;
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        for seg in &self.tail {
            draw(SNAKE_COLOR, seg.x, seg.y, 1, 1, c, g);
        }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}
