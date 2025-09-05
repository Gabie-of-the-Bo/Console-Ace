pub enum Play {
    Highest(usize),
    Pair(usize),
    DoublePair(usize, usize),
    ThreeOfAKind(usize),
    Straight(usize),
    Flush,
    FullHouse(usize, usize), // Pair, Three of a kind
    Poker(usize),
    StraightFlush(usize),
    RoyalFlush
}