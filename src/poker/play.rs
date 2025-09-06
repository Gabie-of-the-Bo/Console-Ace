use std::collections::{HashMap, HashSet};

use crate::poker::card::Card;

#[derive(PartialEq, Debug)]
pub enum Play {
    Highest(usize),
    Pair(usize),
    DoublePair(usize, usize),
    ThreeOfAKind(usize),
    Straight(usize),
    Flush,
    FullHouse(usize, usize), // Pair, Three of a kind
    FourOfAKind(usize),
    StraightFlush(usize),
    RoyalFlush
}

pub fn analyze_play(hand: &Vec<Card>, community: &Vec<Card>) -> Play {
    let all = hand.iter().chain(community).collect::<Vec<_>>();

    // Create number map
    let mut numbers = HashMap::<_, Vec<_>>::new();

    for c in all.iter() {
        numbers.entry(c.value()).or_default().push(*c);
    }

    // RoyalFlush / StraightFlush / Straight
    let mut straights = (2..=10).rev()
        .map(|i| (i..i + 5).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    straights.push(vec!(14, 2, 3, 4, 5)); // Ace-low straight
    
    let mut straight_found = None;

    for ns in straights {
        let has_all = ns.iter().all(|i| numbers.contains_key(&i));

        if has_all {
            let suits = ns.iter()
                .map(|i| numbers.get(&i).unwrap())
                .map(|s| s.iter().map(|c| c.suit.clone()).collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).cloned().collect())
                .unwrap();

            if suits.is_empty() {
                if straight_found.is_none() {
                    straight_found = Some(ns[4]);
                }

            } else if ns[4] == 14 {
                return Play::RoyalFlush;
            
            } else {
                return Play::StraightFlush(ns[4]);
            }
        }
    }

    // Four of a kind
    let four = numbers.iter()
        .filter(|(_, cs)|cs.len() == 4)
        .max_by_key(|(i, _)| *i);

    if let Some((v, _)) = four {
        return Play::FourOfAKind(*v);
    }    

    // Flush (used later)
    let mut suits = HashMap::<_, i32>::new();

    for c in all.iter() {
        *suits.entry(c.suit.clone()).or_default() += 1;
    }

    let flush = suits.iter().any(|(_, &i)| i >= 5);

    // FullHouse / ThreeOfAKind
    let three = numbers.iter()
        .filter(|(_, cs)|cs.len() == 3)
        .max_by_key(|(i, _)| *i);

    if let Some((t, _)) = three {
        let p = numbers.iter()
            .filter(|(_, cs)| cs.len() >= 2)
            .filter(|&(i, _)| i != t)
            .map(|i| *i.0)
            .max();

        if let Some(i) = p {
            return Play::FullHouse(*t, i);
        }

        if flush {
            return Play::Flush;
        }

        return Play::ThreeOfAKind(*t);
    }

    if flush {
        return Play::Flush;
    }

    if let Some(s) = straight_found {
        return Play::Straight(s);
    }

    // DoublePair / Pair
    let mut pairs = numbers.iter()
        .filter(|(_, cs)| cs.len() == 2)
        .map(|i| *i.0)
        .collect::<Vec<_>>();

    pairs.sort();

    if pairs.len() >= 2 {
        let num_pairs = pairs.len();

        return Play::DoublePair(pairs[num_pairs - 1], pairs[num_pairs - 2]);
    }

    if pairs.len() == 1 {
        return Play::Pair(pairs[0]);
    }

    // Return highest card
    Play::Highest(all.iter().max_by_key(|c| c.value()).unwrap().value())
}