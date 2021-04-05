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

mod deck;
use deck::*;

fn main() {
    let mut deck = Deck::new();
    let mut hand = Hand::new();
    let mut dealer_hand = Hand::new();

    hand.add(&mut deck);
    hand.add(&mut deck);
    dealer_hand.add(&mut deck);
    dealer_hand.add(&mut deck);

    while (hand.sum().0 < 21) && (hand.sum().1 != 21) {
        dealer_hand.print_dealer();
        hand.print();
        println!("Do you want to continue? [Y/n]");
        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .expect("Could not read answer");
        match answer.as_str() {
            "\n" | "y\n" | "Y\n" => {
                hand.add(&mut deck);
                if dealer_hand.sum().0 < 16 {
                    dealer_hand.add(&mut deck);
                }
            }
            "n\n" | "N\n" => {
                if dealer_hand.sum().0 < 16 {
                    dealer_hand.add(&mut deck);
                }
                break;
            }
            _ => continue,
        }
    }

    if (hand.sum().0 == 21) || (hand.sum().1 == 21) {
        println!("You got 21; you win!");
    } else if hand.sum().0 > 21 {
        println!("You busted; you lose!");
    } else if dealer_hand.sum().0 > 21 {
        println!("The dealer busted; you win!");
    } else if (dealer_hand.sum().0 == 21) || (dealer_hand.sum().1 == 21) {
        println!("The dealer got 21; you lose!");
    } else if dealer_hand.sum().0 > hand.sum().0 {
        println!("The dealer got closer to 21; the dealer wins!");
    } else {
        println!("You got closer to 21 than the dealer did; you win!");
    }
}
