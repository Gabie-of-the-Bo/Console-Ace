use std::collections::HashSet;

use rand::{rng, seq::IndexedRandom};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::poker::{card::Card, deck::Deck, play::analyze_play};

pub fn monte_carlo_likeliness_to_win(hand: &[Card], community: &[Card], num_players: usize, iters: usize) -> f32 {
    let unknowns = 7 - (hand.len() + community.len());

    let all = hand.iter().chain(community).collect::<Vec<_>>();
    let deck = Deck::new();
    
    let available_cards = deck.cards.par_iter()
        .filter(|a| !all.iter().any(|b| a.suit == b.suit && a.number == b.number))
        .collect::<Vec<_>>();

    let equity = (0..iters).into_par_iter().map(|_| {
        // Shuffle available cards
        let mut rng = rng();

        let mut available_clone = available_cards.choose_multiple(&mut rng, unknowns + 2 * (num_players - 1))
            .cloned()
            .collect::<Vec<_>>();

        // Get possible community card set
        let mut new_community = community.to_vec();
        new_community.extend(available_clone.drain(0..unknowns).cloned());

        // Get possible hands
        let mut hands = vec!(hand.to_vec());

        for _ in 0..(num_players - 1) {
            hands.push(available_clone.drain(0..2).cloned().collect());
        }

        // Check who won
        let mut plays = (0..num_players)
            .map(|i| analyze_play(&hands[i], &new_community))
            .enumerate()
            .collect::<Vec<_>>();

        plays.sort_unstable_by(|a, b| a.1.cmp(&b.1));

        let best_play = &plays.last().unwrap().1;

        let tied_best = plays.iter()
            .filter(|i| i.1 == *best_play)
            .map(|i| i.0)
            .collect::<HashSet<_>>();

        if tied_best.contains(&0) {
            1.0 / tied_best.len() as f32

        } else {
            0.0
        }
    })
    .sum::<f32>();

    equity / iters as f32
}