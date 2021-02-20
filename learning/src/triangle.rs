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

#![allow(dead_code)]

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Triangle {
    pub sides: [[(char, Option<f64>); 3]; 2],
    pub angles: [[(char, Option<f64>); 3]; 2],
    pub area: [Option<f64>; 2],
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            sides: [
                [('a', None), ('b', None), ('c', None)],
                [('a', None), ('b', None), ('c', None)],
            ],
            angles: [
                [('A', None), ('B', None), ('C', None)],
                [('A', None), ('B', None), ('C', None)],
            ],
            area: [None, None],
        }
    }

    pub fn add_angles(&mut self, angles: Vec<(char, Option<f64>)>) -> Self {
        for i in 0..angles.len() {
            self.angles[0][i] = (angles[i].0, angles[i].1);
        }
        Self { ..*self }
    }

    pub fn add_sides(&mut self, sides: Vec<(char, Option<f64>)>) -> Self {
        for i in 0..sides.len() {
            self.sides[0][i] = (sides[i].0, sides[i].1);
        }
        Self { ..*self }
    }

    pub fn build(&mut self) -> Option<Self> {
        if self.sides[0].iter().all(|&x| x.1.is_some()) {
            // sss
            if (self.sides[0][0].1.unwrap() + self.sides[0][1].1.unwrap())
                <= self.sides[0][2].1.unwrap()
            {
                println!(
                    "This is not a valid triangle, since {}({}) + {}({}) <= {}({})",
                    self.sides[0][0].0,
                    self.angles[0][0].1.unwrap(),
                    self.sides[0][1].0,
                    self.sides[0][1].1.unwrap(),
                    self.sides[0][2].0,
                    self.sides[0][2].1.unwrap()
                );
                return None;
            } else if (self.sides[0][1].1.unwrap() + self.sides[0][2].1.unwrap())
                <= self.sides[0][0].1.unwrap()
            {
                println!(
                    "This is not a valid triangle, since {}({}) + {}({}) <= {}({})",
                    self.sides[0][1].0,
                    self.sides[0][1].1.unwrap(),
                    self.sides[0][2].0,
                    self.sides[0][2].1.unwrap(),
                    self.sides[0][0].0,
                    self.sides[0][0].1.unwrap()
                );
                return None;
            } else if (self.sides[0][2].1.unwrap() + self.sides[0][0].1.unwrap())
                <= self.sides[0][1].1.unwrap()
            {
                println!(
                    "This is not a valid triangle, since {}({}) + {}({}) <= {}({})",
                    self.sides[0][2].0,
                    self.sides[0][2].1.unwrap(),
                    self.sides[0][0].0,
                    self.sides[0][0].1.unwrap(),
                    self.sides[0][1].0,
                    self.sides[0][1].1.unwrap()
                );
                return None;
            }

            self.angles[0][0].1 = Some(
                ((self.sides[0][1].1.unwrap().powf(2.) + self.sides[0][2].1.unwrap().powf(2.)
                    - self.sides[0][0].1.unwrap().powf(2.))
                    / (2. * self.sides[0][1].1.unwrap() * self.sides[0][2].1.unwrap()))
                .acos()
                .to_degrees(),
            );
            self.angles[0][1].1 = Some(
                ((self.sides[0][2].1.unwrap().powf(2.) + self.sides[0][0].1.unwrap().powf(2.)
                    - self.sides[0][1].1.unwrap().powf(2.))
                    / (2. * self.sides[0][2].1.unwrap() * self.sides[0][0].1.unwrap()))
                .acos()
                .to_degrees(),
            );
            self.angles[0][2].1 =
                Some(180. - self.angles[0][0].1.unwrap() - self.angles[0][1].1.unwrap());
            self.area[0] = Some(
                0.5 * self.sides[0][0].1.unwrap()
                    * self.sides[0][1].1.unwrap()
                    * self.angles[0][2].1.unwrap().to_radians().sin(),
            );
        } else if self.sides[0][0].1.is_some()
            && self.angles[0][1].1.is_some()
            && self.sides[0][2].1.is_some()
        {
            // sas
            if self.angles[0][1].1.unwrap() >= 180. {
                println!(
                    "This is not a valid triangle, since {0} >= 180, ({0} = {1})",
                    self.angles[0][1].0,
                    self.angles[0][1].1.unwrap()
                );
                return None;
            }
            self.sides[0][1].1 = Some(
                (self.sides[0][0].1.unwrap().powf(2.) + self.sides[0][2].1.unwrap().powf(2.)
                    - 2. * self.sides[0][0].1.unwrap()
                        * self.sides[0][2].1.unwrap()
                        * self.angles[0][1].1.unwrap().to_radians().cos())
                .sqrt(),
            );
            self.angles[0][0].1 = Some(
                ((self.sides[0][1].1.unwrap().powf(2.) + self.sides[0][2].1.unwrap().powf(2.)
                    - self.sides[0][0].1.unwrap().powf(2.))
                    / (2. * self.sides[0][1].1.unwrap() * self.sides[0][2].1.unwrap()))
                .acos()
                .to_degrees(),
            );
            self.angles[0][2].1 =
                Some(180. - self.angles[0][0].1.unwrap() - self.angles[0][1].1.unwrap());
            Some(
                0.5 * self.sides[0][0].1.unwrap()
                    * self.sides[0][1].1.unwrap()
                    * self.angles[0][2].1.unwrap().to_radians().sin(),
            );
        } else if self.sides[0][0..2].iter().all(|&x| x.1.is_some())
            && self.angles[0][0].1.is_some()
        {
            // ssa
            if self.angles[0][0].1.unwrap() >= 180. {
                println!(
                    "This is not a valid triangle, since {0} >= 180, ({0} = {1})",
                    self.angles[0][0].0,
                    self.angles[0][0].1.unwrap()
                );
                return None;
            }
            self.angles[0][1].1 = Some(
                ((self.sides[0][1].1.unwrap() * self.angles[0][0].1.unwrap().to_radians().sin())
                    / self.sides[0][0].1.unwrap())
                .asin()
                .to_degrees(),
            );
            self.angles[0][2].1 =
                Some(180. - self.angles[0][0].1.unwrap() - self.angles[0][1].1.unwrap());
            self.sides[0][2].1 = Some(
                (self.sides[0][0].1.unwrap() * self.angles[0][2].1.unwrap().to_radians().sin())
                    / self.angles[0][0].1.unwrap().to_radians().sin(),
            );
            self.area[0] = Some(
                0.5 * self.sides[0][0].1.unwrap()
                    * self.sides[0][1].1.unwrap()
                    * self.angles[0][2].1.unwrap().to_radians().sin(),
            );
            if ((180. - self.angles[0][1].1.unwrap()) + self.angles[0][0].1.unwrap()) < 180. {
                for i in 0..3 {
                    self.angles[1][i] = self.angles[0][i];
                    self.sides[1][i] = self.sides[0][i];
                }
                self.angles[1][1].1 = Some(180. - self.angles[0][1].1.unwrap());
                self.angles[1][2].1 =
                    Some(180. - self.angles[1][0].1.unwrap() - self.angles[1][1].1.unwrap());
                self.sides[1][2].1 = Some(
                    (self.sides[1][0].1.unwrap() * self.angles[1][2].1.unwrap().to_radians().sin())
                        / self.angles[1][0].1.unwrap().to_radians().sin(),
                );
                self.area[1] = Some(
                    0.5 * self.sides[1][0].1.unwrap()
                        * self.sides[1][1].1.unwrap()
                        * self.angles[1][2].1.unwrap().to_radians().sin(),
                )
            }
        } else if self.angles[0][0].1.is_some()
            && self.sides[0][1].1.is_some()
            && self.angles[0][2].1.is_some()
        {
            // asa
            self.angles[0][2].1 =
                Some(180. - self.angles[0][0].1.unwrap() - self.angles[0][1].1.unwrap());
            self.sides[0][0].1 = Some(
                (self.sides[0][2].1.unwrap() * self.angles[0][0].1.unwrap().to_radians().sin())
                    / self.angles[0][2].1.unwrap().to_radians().sin(),
            );
            self.sides[0][1].1 = Some(
                (self.sides[0][2].1.unwrap() * self.angles[0][1].1.unwrap().to_radians().sin())
                    / self.angles[0][2].1.unwrap().to_radians().sin(),
            );
            Some(
                0.5 * self.sides[0][0].1.unwrap()
                    * self.sides[0][1].1.unwrap()
                    * self.angles[0][2].1.unwrap().to_radians().sin(),
            );
        } else if self.angles[0][0..2].iter().all(|&x| x.1.is_some())
            && self.sides[0][0].1.is_some()
        {
            // aas
            self.angles[0][2].1 =
                Some(180. - self.angles[0][0].1.unwrap() - self.angles[0][1].1.unwrap());
            self.sides[0][1].1 = Some(
                (self.sides[0][0].1.unwrap() * self.angles[0][1].1.unwrap().to_radians().sin())
                    / self.angles[0][0].1.unwrap().to_radians().sin(),
            );
            self.sides[0][2].1 = Some(
                (self.sides[0][0].1.unwrap() * self.angles[0][2].1.unwrap().to_radians().sin())
                    / self.angles[0][0].1.unwrap().to_radians().sin(),
            );
            Some(
                0.5 * self.sides[0][0].1.unwrap()
                    * self.sides[0][1].1.unwrap()
                    * self.angles[0][2].1.unwrap().to_radians().sin(),
            );
        } else if self.angles[0].iter().all(|&x| x.1.is_some()) {
            // aaa
            println!("A side is needed");
            return None;
        } else {
            // other
            println!("unknown");
            return None;
        }
        Some(Self { ..*self })
    }
}
