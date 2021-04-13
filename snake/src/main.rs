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

use ncurses::*;
use rand::{thread_rng, Rng};

const SNAKE_BODY: char = 'O';
const SNAKE_FOOD: char = '@';
const WIDTH: i32 = 40;
const HEIGHT: i32 = 20;

const KEY_W_UPPERCASE: i32 = 87;
const KEY_W_LOWERCASE: i32 = 119;
const KEY_A_UPPERCASE: i32 = 65;
const KEY_A_LOWERCASE: i32 = 97;
const KEY_S_UPPERCASE: i32 = 83;
const KEY_S_LOWERCASE: i32 = 115;
const KEY_D_UPPERCASE: i32 = 68;
const KEY_D_LOWERCASE: i32 = 100;

const KEY_UP: i32 = 259;
const KEY_DOWN: i32 = 258;
const KEY_LEFT: i32 = 260;
const KEY_RIGHT: i32 = 261;
const KEY_ESC: i32 = 27;

#[derive(PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    DOWN,
    UP,
    STILL,
}

#[derive(Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

struct Snake {
    position: Position,
    length: u32,
    direction: Direction,
    tail: [Position; ((WIDTH * HEIGHT) - 1) as usize],
}

fn main() {
    initscr();
    timeout(0);
    cbreak();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    nodelay(stdscr(), true);

    let mut snake = Snake {
        position: {
            Position {
                x: WIDTH / 2,
                y: HEIGHT / 2,
            }
        },
        length: 1,
        direction: Direction::STILL,
        tail: {
            [Position {
                x: WIDTH / 2,
                y: HEIGHT / 2,
            }; ((WIDTH * HEIGHT) - 1) as usize]
        },
    };
    let mut food = Position {
        x: thread_rng().gen_range(0..WIDTH),
        y: thread_rng().gen_range(0..HEIGHT),
    };

    'Main: loop {
        match getch() {
            KEY_W_UPPERCASE | KEY_W_LOWERCASE | KEY_UP => {
                if snake.direction != Direction::DOWN {
                    snake.direction = Direction::UP;
                }
            }
            KEY_A_UPPERCASE | KEY_A_LOWERCASE | KEY_LEFT => {
                if snake.direction != Direction::RIGHT {
                    snake.direction = Direction::LEFT;
                }
            }
            KEY_S_UPPERCASE | KEY_S_LOWERCASE | KEY_DOWN => {
                if snake.direction != Direction::UP {
                    snake.direction = Direction::DOWN;
                }
            }
            KEY_D_UPPERCASE | KEY_D_LOWERCASE | KEY_RIGHT => {
                if snake.direction != Direction::LEFT {
                    snake.direction = Direction::RIGHT;
                }
            }
            KEY_ESC => break 'Main,
            _ => {}
        };

        match snake.direction {
            Direction::LEFT => {
                snake.position.x -= 1;
            }
            Direction::RIGHT => {
                snake.position.x += 1;
            }
            Direction::DOWN => {
                snake.position.y += 1;
            }
            Direction::UP => {
                snake.position.y -= 1;
            }
            Direction::STILL => {}
        }

        if snake.position.x == food.x && snake.position.y == food.y {
            snake.length += 1;
            food.x = thread_rng().gen_range(0..WIDTH);
            food.y = thread_rng().gen_range(0..HEIGHT);
        }

        if snake.position.x < 0
            || snake.position.x >= WIDTH
            || snake.position.y < 0
            || snake.position.y >= HEIGHT
        {
            break;
        }

        let mut x1 = snake.tail[0].x;
        let mut y1 = snake.tail[0].y;
        snake.tail[0].x = snake.position.x;
        snake.tail[0].y = snake.position.y;
        let mut x2: i32;
        let mut y2: i32;

        for i in 1..snake.length as usize {
            x2 = snake.tail[i].x;
            y2 = snake.tail[i].y;
            snake.tail[i].x = x1;
            snake.tail[i].y = y1;
            x1 = x2;
            y1 = y2;
        }

        erase();

        for y in -1..=HEIGHT {
            for x in -1..=WIDTH {
                if x < WIDTH && y < HEIGHT && x >= 0 && y >= 0 {
                    if x == snake.position.x && y == snake.position.y {
                        addch(SNAKE_BODY as u64);
                    } else if x == food.x && y == food.y {
                        addch(SNAKE_FOOD as u64);
                    } else {
                        let mut did_print: bool = false;
                        for i in 0..snake.length as usize {
                            if snake.position.x == snake.tail[i].x
                                && snake.position.y == snake.tail[i].y
                                && i != 0
                            {
                                break 'Main;
                            } else if x == snake.tail[i].x && y == snake.tail[i].y {
                                addch(SNAKE_BODY as u64);
                                did_print = true;
                                break;
                            }
                        }
                        if !did_print {
                            addch(' ' as u64);
                        }
                    }
                } else {
                    addch('*' as u64);
                }
            }
            addch('\n' as u64);
        }

        addstr(format!("Press ESC to Quit\tScore {}", snake.length).as_str());
        refresh();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    clear();
    addstr(format!("Final Score {}", snake.length).as_str());
    refresh();
    timeout(-1);
    getch();
    endwin();
}
