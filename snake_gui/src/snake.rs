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

use crate::draw::{draw, BLOCK_SIZE};
use crate::game::{Color, Position};
use piston_window::{Context, G2d, Size};
use std::collections::VecDeque;

const SNAKE_WAIT: f64 = 0.2;

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
    pub length: u32,
    pub direction: Direction,
    pub tail: VecDeque<Position>,
    is_alive: bool,
    wait: f64,
}

impl Snake {
    pub fn new(x: u32, y: u32) -> Snake {
        Snake {
            position: Position { x: x, y: y },
            length: 1,
            direction: Direction::Still,
            tail: vec![].into_iter().collect(),
            is_alive: true,
            wait: 0.0,
        }
    }

    fn is_valid(&self, size: Size) -> bool {
        let x = self.position.x;
        let y = self.position.y;
        x > 0
            && y > 0
            && x < (size.width / BLOCK_SIZE) as u32
            && y < (size.height / BLOCK_SIZE) as u32
    }

    pub fn mv(&mut self, size: Size, direction: Direction) {
        self.wait = 0.0;
        if !self.is_valid(size) {
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
                if self.overlap_tail(self.position.x - 1, self.position.y) {
                    self.is_alive = false;
                    return;
                }
                self.position.x -= 1;
                self.tail.pop_back();
                self.tail.push_front(self.position);
            }
            Direction::Right => {
                if self.overlap_tail(self.position.x + 1, self.position.y) {
                    self.is_alive = false;
                    return;
                }
                self.position.x += 1;
                self.tail.pop_back();
                self.tail.push_front(self.position);
            }
            Direction::Up => {
                if self.overlap_tail(self.position.x, self.position.y - 1) {
                    self.is_alive = false;
                    return;
                }
                self.position.y -= 1;
                self.tail.pop_back();
                self.tail.push_front(self.position);
            }
            Direction::Down => {
                if self.overlap_tail(self.position.x, self.position.y + 1) {
                    self.is_alive = false;
                    return;
                }
                self.position.y += 1;
                self.tail.pop_back();
                self.tail.push_front(self.position);
            }
            Direction::Still => {}
        }
    }

    fn overlap_tail(&self, x: u32, y: u32) -> bool {
        self.tail.contains(&Position { x, y })
    }

    pub fn eat(&mut self) {
        match self.direction {
            Direction::Left => {
                self.tail.push_back(Position {
                    x: self.position.x + 1,
                    y: self.position.y,
                });
            }
            Direction::Right => {
                self.tail.push_back(Position {
                    x: self.position.x - 1,
                    y: self.position.y,
                });
            }
            Direction::Up => {
                self.tail.push_back(Position {
                    x: self.position.x,
                    y: self.position.y + 1,
                });
            }
            Direction::Down => {
                self.tail.push_back(Position {
                    x: self.position.x,
                    y: self.position.y - 1,
                });
            }
            Direction::Still => {}
        }
        self.length += 1;
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        draw(
            Color::SNAKE_HEAD,
            self.position.x as u32,
            self.position.y as u32,
            1,
            1,
            c,
            g,
        );
        self.tail
            .iter()
            .skip(1)
            .for_each(|seg| draw(Color::SNAKE_BODY, seg.x as u32, seg.y as u32, 1, 1, c, g));
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn update(&mut self, size: Size, dt: f64, direction: Direction) {
        self.wait += dt;
        if self.wait > SNAKE_WAIT {
            self.mv(size, direction);
        }
    }
}
