use crate::poker::card::Card;

pub const SMALL_BLIND: usize = 2;
pub const BIG_BLIND: usize = 5;

pub struct Player {
    pub name: String,
    pub money: usize,
    pub bet: usize,
    pub hand: Vec<Card>
}

impl Player {
    pub fn new(name: String, money: usize) -> Self {
        Player { name, money, bet: 0, hand: vec!() }
    }

    pub fn give_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn bet_chips(&mut self, chips: usize) {
        self.money -= chips;
        self.bet += chips;
    }

    pub fn take_bet(&mut self) {
        self.money += self.bet;
        self.bet = 0;
    }

    pub fn lose_bet(&mut self) {
        self.bet = 0;
    }
}