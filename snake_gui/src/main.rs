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

mod draw;
mod food;
mod game;
mod screen;
mod snake;

use food::Food;
use game::Game;
use piston_window::*;
use screen::Screen;
use snake::Snake;

fn main() {
    let mut game = Game::new(
        Snake::new(Screen::WIDTH / 2, Screen::HEIGHT / 2),
        Food::new(),
    );
    let mut window: PistonWindow = WindowSettings::new(
        "Snake Game",
        [
            Screen::WIDTH * Screen::WIDTH,
            Screen::HEIGHT * Screen::HEIGHT,
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let mut x = Key::Q;

    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(k)) = e.press_args() {
            if k != x {
                x = k;
            }
        }

        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 0.5], g);
            game.draw(&c, g);
        });

        e.update(|args| {
            game.update(args, x);
        });

        if game.over() {
            if let Some(args) = e.render_args() {
                game.draw_game_over(args);
            }
        }
    }
}
