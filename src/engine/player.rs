use crate::{actor::actor::PokerActor, poker::card::Card};

pub const SMALL_BLIND: usize = 2;
pub const BIG_BLIND: usize = 5;

pub struct Player {
    pub name: String,
    pub money: usize,
    pub bet: usize,
    pub folded: bool,
    pub hand: Vec<Card>,
    pub actor: Box<dyn PokerActor>
}

impl Player {
    pub fn new(name: String, money: usize, actor: Box<dyn PokerActor>) -> Self {
        Player { name, money, bet: 0, folded: false, hand: vec!(), actor }
    }

    pub fn give_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn bet_chips(&mut self, chips: usize) {
        self.money -= chips;
        self.bet += chips;
    }

    pub fn win(&mut self, chips: usize) {
        self.money += chips;
    }

    pub fn lose_bet(&mut self) {
        self.bet = 0;
    }

    pub fn fold(&mut self) {
        self.folded = true;
    }

    pub fn unfold(&mut self) {
        self.folded = false;
    }
}