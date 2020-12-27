mod deck;
use deck::*;

fn main() {
    play();
}

fn play() {
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
        std::io::stdin().read_line(&mut answer).expect("Could not read answer");
        match answer.as_str() {
            "\n"|"y\n"|"Y\n" => {
                hand.add(&mut deck);
                if dealer_hand.sum().1 < 17 {
                    dealer_hand.add(&mut deck);
                }
            },
            "n\n"|"N\n" => break,
            _ => continue
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
