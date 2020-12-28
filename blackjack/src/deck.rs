pub struct Card(pub String, pub u8);
pub struct Hand {pub card: Vec<Card>}
pub struct Deck {pub card: Vec<Card>}

impl Card {
    pub fn new(name: &str, value: u8) -> Card {
        Card(String::from(name), value)
    }
}

impl Hand {
    pub fn new() -> Hand {
        Hand {card:vec![]}
    }
    pub fn add(&mut self, deck: &mut Deck) {
        use rand::Rng;
        self.card.insert(self.card.len(), deck.card.swap_remove(rand::thread_rng().gen_range(0..deck.card.len())));
    }
    fn is_ace(&mut self) -> u8 {
        let mut sum: u8 = 0;
        for i in self.card.iter() {
            if i.1 == 1 {
                sum += 1;
            }
        }
        sum
    }
    pub fn sum(&mut self) -> (u8,u8) {
        let mut sum: (u8,u8) = (0,0);
        for i in self.card.iter() {
            sum.0 += i.1;
        }
        if self.is_ace() > 1 {
            sum.1 = sum.0 + 10;
        }
        sum
    }
    pub fn print(&mut self) {
        println!("Your current cards add up to {}; the cards are:", {
            if self.is_ace() == 1 {
                if self.sum().1 < 22 {
                    format!("{} if your ace is a 1 or {} if your ace is an 11", self.sum().0, self.sum().1)
                }
                else {
                    format!("{}", self.sum().0)
                }
            } else if self.is_ace() > 1 {
                if self.sum().1 < 22 {
                    format!("{} since you can only have 1 ace that is worth 11 points", self.sum().1)
                } else {
                    format!("{}", self.sum().0)
                }
            } else {
                format!("{}", self.sum().0)
            }
        });
        for i in self.card.iter() {
            println!("{}", i.0);
        }
    }
    pub fn print_dealer(&mut self) {
        println!("The dealer has {} cards and they are showing you that they have {} {}.", self.card.len(),
            match self.card.get(0).unwrap().1 {
                1|8 => "an",
                _ => "a"
            }, self.card.get(0).unwrap().0);
    }
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck {card: vec![]};
        let suits = ["diamonds", "spades", "clubs", "hearts"];
        let values = [("ace", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9), ("ten", 10), ("jack", 10), ("queen", 10), ("king", 10)];
        for i in 0..4 {
            for o in 0..13 {
                deck.card.push(Card::new(format!("{} of {}", values[o].0, suits[i]).as_str(), values[o].1));
            }
        }
        deck
    }
}
