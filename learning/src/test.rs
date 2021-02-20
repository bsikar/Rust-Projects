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

#[cfg(test)]
mod tests {
    use crate::Triangle;

    #[test]
    fn sss1() {
        let triangle = Triangle::new()
            .add_sides(vec![('a', Some(12.)), ('b', Some(15.)), ('c', Some(12.))])
            .build()
            .unwrap();
        let out = Triangle {
            sides: [
                [('a', Some(12.0)), ('b', Some(15.0)), ('c', Some(12.0))],
                [('a', None), ('b', None), ('c', None)],
            ],
            angles: [
                [
                    ('A', Some(51.31781254651057)),
                    ('B', Some(77.36437490697888)),
                    ('C', Some(51.31781254651055)),
                ],
                [('A', None), ('B', None), ('C', None)],
            ],
            area: [Some(70.25622748198197), None],
        };
        assert_eq!(triangle, out);
    }

    #[test]
    fn sss2() {
        let triangle = Triangle::new()
            .add_sides(vec![('a', Some(3.)), ('b', Some(2.)), ('c', Some(1.))])
            .build();
        assert_eq!(triangle, None);
    }

    #[test]
    fn sas1() {
        let triangle = Triangle::new()
            .add_sides(vec![('a', Some(12.)), ('b', None), ('c', Some(4.))])
            .add_angles(vec![('A', None), ('B', Some(36.)), ('C', None)])
            .build()
            .unwrap();
        let out = Triangle {
            sides: [
                [
                    ('a', Some(12.0)),
                    ('b', Some(9.07382876959914)),
                    ('c', Some(4.0)),
                ],
                [('a', None), ('b', None), ('c', None)],
            ],
            angles: [
                [
                    ('A', Some(128.98260190586)),
                    ('B', Some(36.0)),
                    ('C', Some(15.017398094139992)),
                ],
                [('A', None), ('B', None), ('C', None)],
            ],
            area: [None, None],
        };

        assert_eq!(triangle, out);
    }

    #[test]
    fn sas2() {
        let triangle = Triangle::new()
            .add_sides(vec![('a', Some(12.)), ('b', None), ('c', Some(4.))])
            .add_angles(vec![('A', None), ('B', Some(180.)), ('C', None)])
            .build();
        assert_eq!(triangle, None);
    }

    #[test]
    fn sas3() {
        let triangle = Triangle::new()
            .add_sides(vec![('a', Some(12.)), ('b', None), ('c', Some(4.))])
            .add_angles(vec![('A', None), ('B', Some(200.)), ('C', None)])
            .build();
        assert_eq!(triangle, None);
    }
    // ssa asa aas aaa
}
