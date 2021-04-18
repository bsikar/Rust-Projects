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

fn main() {
    let ar = [-51, -11, 3, 13, 17, 19, 22, 29, 44, 57, 62, 78];

    binary_search(&ar, -441);
    binary_search(&ar, -1);
    binary_search(&ar, 15);
    binary_search(&ar, 34);
    binary_search(&ar, 45);
    binary_search(&ar, 62);
    binary_search(&ar, 75);
    println!(
        "{}",
        std::iter::successors(Some(5480u16), |&x| if x > 1 {
            x.checked_div(2)
        } else {
            None
        })
        .count()
    );
}

fn binary_search(ar: &[isize], target: isize) {
    let mut min = 0;
    let mut max = ar.len() - 1;
    let mut x = false;
    let mut i = 0;

    //  println!("low mid high");
    while min <= max {
        i += 1;
        let mid = min + (max - min) / 2;
        //        println!("{}   {}   {}", min, mid, max);
        if ar[mid] == target {
            x = true;
            break;
        } else if ar[mid] < target {
            min = mid + 1;
        } else if mid == 0 {
            break;
        } else {
            max = mid - 1;
        }
    }
    println!("{}, {}", i, if x { "found" } else { "not found" });
}
