use crate::poker::card::Card;

pub struct Player {
    pub name: String,
    pub money: usize,
    pub hand: Vec<Card>
}

impl Player {
    pub fn new(name: String, money: usize) -> Self {
        Player { name, money, hand: vec!() }
    }

    pub fn give_card(&mut self, card: Card) {
        self.hand.push(card);
    }
}