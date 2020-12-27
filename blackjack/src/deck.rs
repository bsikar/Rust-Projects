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
        Deck {
            card:
            vec![
                Card::new("ace of diamonds", 1),
                Card::new("two of diamonds", 2),
                Card::new("three of diamonds", 3),
                Card::new("four of diamonds", 4),
                Card::new("five of diamonds", 5),
                Card::new("six of diamonds", 6),
                Card::new("seven of diamonds", 7),
                Card::new("eight of diamonds", 8),
                Card::new("nine of diamonds", 9),
                Card::new("ten of diamonds", 10),
                Card::new("jack of diamonds", 10),
                Card::new("queen of diamonds", 10),
                Card::new("king of diamonds", 10),
                
                Card::new("ace of hearts", 1),
                Card::new("two of hearts", 2),
                Card::new("three of hearts", 3),
                Card::new("four of hearts", 4),
                Card::new("five of hearts", 5),
                Card::new("six of hearts", 6),
                Card::new("seven of hearts", 7),
                Card::new("eight of hearts", 8),
                Card::new("nine of hearts", 9),
                Card::new("ten of hearts", 10),
                Card::new("jack of hearts", 10),
                Card::new("queen of hearts", 10),
                Card::new("king of hearts", 10),
                
                Card::new("ace of spades", 1),
                Card::new("two of spades", 2),
                Card::new("three of spades", 3),
                Card::new("four of spades", 4),
                Card::new("five of spades", 5),
                Card::new("six of spades", 6),
                Card::new("seven of spades", 7),
                Card::new("eight of spades", 8),
                Card::new("nine of spades", 9),
                Card::new("ten of spades", 10),
                Card::new("jack of spades", 10),
                Card::new("queen of spades", 10),
                Card::new("king of spades", 10),
                
                Card::new("ace of clubs", 1),
                Card::new("two of clubs", 2),
                Card::new("three of clubs", 3),
                Card::new("four of clubs", 4),
                Card::new("five of clubs", 5),
                Card::new("six of clubs", 6),
                Card::new("seven of clubs", 7),
                Card::new("eight of clubs", 8),
                Card::new("nine of clubs", 9),
                Card::new("ten of clubs", 10),
                Card::new("jack of clubs", 10),
                Card::new("queen of clubs", 10),
                Card::new("king of clubs", 10)
            ]
        }
    }
}
