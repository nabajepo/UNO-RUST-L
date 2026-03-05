# UNO-RUST-L – UNO Card Game (Rust CLI)

## Overview
UNO-RUST-L is a **command-line implementation of the UNO card game** written in **Rust**.

The program supports multiple players and implements core UNO mechanics such as:
- drawing cards
- playing cards based on color/value
- turn rotation
- special cards (`SKIP`, `REVERSE`, `+2`, `COLOR CHANGE`, `+4`)
- UNO rule handling (when a player has 1 card left)

---

## Features
- Multi-player support (user chooses number of players)
- Randomized deck generation and shuffling
- Full turn management (forward/backward order)
- Special card effects:
  - `SKIP🚫`
  - `REVERSE🔄`
  - `+2`
  - `COLOR-CHANGE🎨`
  - `+4🌈`
- UNO call logic (penalty if UNO is not said)
- Ranking system (stores winners in order)

---

## Technologies
- Rust
- `rand` (for shuffling)
- `chrono` (for timestamp display)

---

## Project Structure
The Rust project is located in the `UNO-GAME/` folder
<img width="179" height="189" alt="image" src="https://github.com/user-attachments/assets/6c5ee504-71a2-46e2-a1dc-08ec66b8eafc" />


---

## How to Run

### Requirements
- Rust installed (`cargo`)

Check:
```bash
rustc --version
cargo --version
cd UNO-GAME
cargo run
```
