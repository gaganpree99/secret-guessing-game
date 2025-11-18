# 🎲 Multiplayer Code Guessing Game (Bulls & Cows Variant)

This is a competitive, turn-based console game written in Rust, designed for 2 or more players. Each player is assigned a unique, secret 4-digit code and must be the first to guess their own code to win.

The game uses a set elimination strategy common in games like Mastermind (or Bulls & Cows), where the system provides feedback on the quality of each guess.

-----

## 🚀 How to Play

### Objective

Be the first player to successfully guess your own unique, secret 4-digit code.

### Setup

1.  Players enter their names.
2.  The system generates a **unique, non-repeating 4-digit code** for each player (e.g., `0485`). The first digit is allowed to be zero.
3.  Players select a starting order, or choose a random start.

### Game Flow

1.  Players take turns entering a 4-digit guess.
2.  The guess is scored against that player's specific secret code.
3.  Feedback is given, and the screen is cleared after a 5-second pause to prevent other players from seeing the secret feedback.
4.  The game continues until a player achieves a winning score (4,4).

-----

## 📊 Scoring and Feedback (X, Y)

The system provides feedback in the format **X, Y**, where:

| Metric | Label | Description |
| :--- | :--- | :--- |
| **X** | **Total Correct Digits** | The count of digits in your guess that are present *anywhere* in your secret code. |
| **Y** | **Digits in Correct Position** | The count of digits that are both correct *and* in the correct spot (**Bulls**). |

**Example:**

  * **Secret Code:** `7846`
  * **Guess:** `6473`
  * **X (Total Correct Digits):** 3 (Digits 6, 4, and 7 are present)
  * **Y (Digits in Correct Position):** 0 (No digit is in the correct spot)
  * **Feedback:** **3,0**

-----

## 🏆 Ranking System

Ranks are determined by the **Round Number** in which a player wins.

  * If multiple players win in the **same round**, they are considered **tied** and share the same rank (e.g., two players finish in Round 3 and are both awarded "2nd Place").
  * The system automatically adjusts the rank for subsequent winners.

The game continues until all players have finished.

-----

## 🛠️ Getting Started (Running the Game)

This game is written in Rust and requires the Rust toolchain to be installed.

### Prerequisites

You must have **Rust** and **Cargo** installed. If you don't, you can install them via [rustup](https://rustup.rs/).

Follow the on-screen prompts to enter player names and guesses!
