# 🂡 Console Ace 🂡

<p align="center">
  <a href="https://gabie-of-the-bo.github.io/Ryna-Language/">
    <img src="https://github.com/Gabie-of-the-Bo/Console-Ace/blob/main/img/capture.png?raw=true" width="60%" style="border-radius: .3em;">
  </a>
</p>

**Console Ace** is a fast, colorful, and surprisingly pretty Texas Hold’em poker game that runs entirely in your terminal. It feels almost like a web app, but it’s pure console magic.  

## ✨ Features
- ♠ **Full Texas Hold’em ruleset** (No-Limit, blinds, side-pots, showdown, etc.)  
- ♥ **Up to 3 AI opponents** (with personality: they bluff, defend, and surprise you)  
- ♦ **Colorful, smooth visuals** — the console has never looked this good  
- ♣ **Fun to play solo** while still challenging  
- 🌐 Planned **LAN multiplayer**

## 🃏 How to Play

- **Community cards** are revealed in the center of the screen (flop, turn, river).  
- **Your seat** is always at the bottom of the screen. Opponent hands stay hidden until showdown.  
- **Dealer button (D)** rotates clockwise after each hand.  
- **Turn indicator (T)** shows whose move it is.  

### Betting Rounds
- First two players post the **blinds** (2 and 5 chips).  
- Once it’s your turn, you’ll see the available options with their shortcut keys:  
  - **Check** → Pass your turn without betting.  
  - **Call** → Match the current bet.  
  - **Raise** → Increase the current bet by at least the minimum raise.  
  - **All-in** → Push all your chips in the middle.  

The game enforces minimum raises, blinds, and side-pot rules just like real Hold’em.

## 🎯 Goal

Outplay, out-bet, and out-bluff your opponents. The last player with chips on the table is the winner.  

## 🚧 Roadmap
- LAN multiplayer support  
- More AI personalities  
- Configurable blinds/stakes  
- More game modes 

## 📦 Installation & Running

```bash
git clone https://github.com/Gabie-of-the-Bo/Console-Ace.git
cd console-ace
cargo run --release
```