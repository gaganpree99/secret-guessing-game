use std::io::{self, Write};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng}; 
use std::{thread, time::Duration};

// --- Type Definitions ---
type Guess = [u8; 4];
// Score is internally represented as (Digits at Correct Position, Digits Correct but Wrong Position)
type Score = (u8, u8); 

// Player struct now holds their unique secret code
#[derive(Debug)] // Required for debugging/printing complex structs
struct Player {
    name: String,
    secret_code: Guess, // Each player has their own secret
    rank: Option<usize>, // Stores the player's finishing position (1st, 2nd, etc.)
}

// --- Core Logic ---

/// Generates a single 4-digit number with non-repeating digits.
/// The first digit is allowed to be 0.
fn generate_secret() -> Guess {
    let mut digits: Vec<u8> = (0..=9).collect();
    let mut rng = thread_rng();

    // Shuffle the digits
    digits.shuffle(&mut rng);

    // Take the first four unique digits. Since they are shuffled, they are non-repeating.
    [digits[0], digits[1], digits[2], digits[3]]
}


/// Calculates the core matching score.
/// Returns (Digits at Correct Position [Y], Digits Correct but Wrong Position).
fn calculate_score(guess: &Guess, secret: &Guess) -> Score {
    let mut correct_position = 0; // Digits at Correct Position (Y)
    let mut total_correct_digits = 0; // Total Correct Digits (X)

    // Use a frequency map for quick checking of digits present in the secret
    let mut secret_counts: [bool; 10] = [false; 10];
    for &digit in secret.iter() {
        secret_counts[digit as usize] = true;
    }

    for i in 0..4 {
        let g_digit = guess[i] as usize;
        let s_digit = secret[i] as usize;

        // Check for Digits at Correct Position (Y)
        if g_digit == s_digit {
            correct_position += 1;
        }

        // Check for Total Matches (X)
        if secret_counts[g_digit] {
            total_correct_digits += 1;
        }
    }

    // Digits Correct but Wrong Position = Total Correct (X) - Correct Position (Y)
    let correct_wrong_position = total_correct_digits - correct_position;

    (correct_position, correct_wrong_position)
}


// --- User Input Helpers ---

/// Clears the console screen using common ANSI escape codes.
fn clear_screen() {
    // ANSI escape code for clearing the screen and moving cursor to home position
    print!("\x1b[2J\x1b[H"); 
    io::stdout().flush().unwrap();
}

/// Reads input from the user.
fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

/// Gets a valid integer input for player count.
fn get_player_count() -> u8 {
    loop {
        print!("Enter the number of players (1 to 10): ");
        io::stdout().flush().unwrap();
        match read_line().parse::<u8>() {
            Ok(n) if n >= 1 && n <= 10 => return n,
            _ => println!("Please enter a number between 1 and 10."),
        }
    }
}

/// Gets a valid 4-digit, non-repeating number guess.
fn get_player_guess(player_name: &str) -> Option<Guess> {
    loop {
        print!("{}, enter your 4-digit guess: ", player_name);
        io::stdout().flush().unwrap();
        let input = read_line();

        if input.len() != 4 {
            println!("Guess must be exactly 4 digits.");
            continue;
        }

        let mut digits: Vec<u8> = Vec::new();
        let mut seen_digits = [false; 10];
        let mut valid = true;

        for (_i, c) in input.chars().enumerate() {
            match c.to_digit(10) {
                Some(d) => {
                    let d_u8 = d as u8;
                    // Check for repetition
                    if seen_digits[d_u8 as usize] {
                        println!("Digits must not be repeated.");
                        valid = false;
                        break;
                    }
                    seen_digits[d_u8 as usize] = true;
                    digits.push(d_u8);
                },
                None => {
                    println!("Input contains non-digit characters.");
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            return Some([digits[0], digits[1], digits[2], digits[3]]);
        }
    }
}

/// Prompts the user to select a starting player index (1-based) or 0 for random.
fn get_starting_player_index(players: &Vec<Player>) -> usize {
    let max_index = players.len();
    loop {
        println!("\n--- Select Starting Player ---");
        // Print player options (1-based index)
        for (i, player) in players.iter().enumerate() {
            println!("  [{}] {}", i + 1, player.name);
        }
        println!("  [0] Random selection");
        print!("Enter selection (0, 1, 2, ...): ");
        io::stdout().flush().unwrap();
        
        let input = read_line();
        match input.parse::<usize>() {
            Ok(0) => {
                let mut rng = thread_rng();
                let random_index = rng.gen_range(0..max_index);
                println!("Randomly selected {} to start!", players[random_index].name);
                return random_index;
            }
            Ok(n) if n >= 1 && n <= max_index => {
                let start_index = n - 1; // Convert 1-based to 0-based
                println!("Starting player is {}.", players[start_index].name);
                return start_index;
            }
            _ => {
                println!("Invalid selection. Please enter 0 for random, or a number corresponding to a player.");
            }
        }
    }
}

/// Displays the post-game menu and handles the winner/game state.
/// Returns true if the game should continue, false to quit or restart.
fn post_game_menu(
    players: &mut Vec<Player>, 
    winner_index: usize, 
    rank_to_assign: usize,
    completed_players: &mut Vec<Player>
) -> bool {
    
    loop {
        println!("\n--- Post-Game Menu ---");
        // Check if we're playing for the LAST spot.
        if players.len() == 1 {
            println!("[1] Finish Game: Assign {} rank and view final menu.", players[winner_index].name);
        } else {
            println!("[1] Continue: Remove {} and play for next place.", players[winner_index].name);
        }
        
        println!("[2] Restart: Start a new game with current players.");
        println!("[3] Quit: Exit the program.");
        print!("Enter your choice (1, 2, or 3): ");
        io::stdout().flush().unwrap();

        match read_line().trim() {
            "1" => {
                // Assign the final rank before removing the player
                if let Some(player) = players.get_mut(winner_index) {
                    player.rank = Some(rank_to_assign);
                }
                
                let winning_player = players.remove(winner_index);
                println!("Removed {} (Rank {}) from active play.", 
                         winning_player.name, 
                         winning_player.rank.unwrap_or(rank_to_assign)
                );
                
                // Move the ranked player to the completed list
                completed_players.push(winning_player);
                
                // Only return false (end game) if the vector is now empty.
                if players.is_empty() {
                    return false; // Signal run_game to break the loop
                }
                return true; // Continue to the next round of the game loop
            }
            "2" => return false, // Signal main to break and restart the whole main function process
            "3" => {
                println!("Thank thank you for playing! Goodbye.");
                std::process::exit(0); // Explicitly exit the program
            }
            _ => {
                println!("Invalid input. Please enter 1, 2, or 3.");
                thread::sleep(Duration::from_secs(1));
                clear_screen();
            }
        }
    }
}

/// Encapsulates the entire game setup and main loop logic for easy restart.
fn run_game() {
    clear_screen();
    println!("--- 🎲 Multiplayer Code Guessing Game (Individual Secrets) ---");
    println!("Each player has a unique, hidden 4-digit code (non-repeating digits, can start with 0).");
    println!("Players take turns guessing their own secret. First to guess wins!");
    
    // 1. Setup Players and Assign Individual Secrets
    let num_players_u8 = get_player_count();
    let num_players = num_players_u8 as usize;
    let mut players: Vec<Player> = Vec::new();

    for i in 0..num_players_u8 {
        print!("Enter name for Player {}: ", i + 1);
        io::stdout().flush().unwrap();
        let name = read_line();
        
        // Generate a unique secret for this player
        let secret_code = generate_secret();
        
        // *** DEBUGGING PRINT ***
        //println!("DEBUG: {}'s Secret Code is: {}{}{}{}", 
                 name, secret_code[0], secret_code[1], secret_code[2], secret_code[3]);
        
        players.push(Player { name, secret_code, rank: None });
    }

    println!("\nAll secret codes have been generated. Let the guessing begin!");

    // 2. Determine Starting Player Index
    let mut current_player_index = get_starting_player_index(&players);

    // *** CLEAR SCREEN ***
    clear_screen();

    // 3. Game Loop Variables
    let mut round_number: u32 = 1; // Tracks full cycles (rounds)
    let mut total_guesses: u32 = 0; // Tracks total guesses across all rounds
    
    // RANKING VARIABLES (For round-based tie ranking)
    let mut rank_to_assign: usize = 1; // The rank for the next *distinct* finisher (1st, 2nd, 3rd...)
    let mut last_assigned_round: u32 = 0; // The round number the most recent rank was achieved in.
    
    // List to hold players who have finished the game
    let mut completed_players: Vec<Player> = Vec::new();

    loop {
        // Handle final player finishing the game
        if players.is_empty() {
             println!("\nAll players have finished the game. Thanks for playing!");
             break;
        }
        
        // Handle the last remaining player (auto-assignment of final rank)
        if players.len() == 1 && players[0].rank.is_none() {
            let last_player_index = 0;
            // The last player automatically gets the current distinct rank
            players[last_player_index].rank = Some(rank_to_assign);
            println!("\n--- Final Player Ranked ---");
            println!("{} is automatically assigned {} place.", 
                     players[last_player_index].name, rank_to_assign);

            // Move the last player to the completed list and break
            completed_players.extend(players.drain(..)); 
            break;
        }
        
        // Ensure the current_player_index is valid after a removal
        if current_player_index >= players.len() {
            current_player_index = 0;
        }

        let current_player = &players[current_player_index];
        
        total_guesses += 1; // Increment guess counter first

        println!("\n======================================");
        println!("ROUND {} | {}'s Guess", round_number, current_player.name);
        println!("======================================");
        
        let guess = match get_player_guess(&current_player.name) {
            Some(g) => g,
            None => { 
                current_player_index = (current_player_index + 1) % players.len();
                continue; // Skip turn if input fails validation
            },
        };

        // 4. Score and Feedback: Use the current player's unique secret code
        let (y_score, c_score) = calculate_score(&guess, &current_player.secret_code);
        
        // Y = Digits at Correct Position
        let y_correct_pos = y_score; 
        
        // X = Total Correct Digits (Y + C)
        let x_total_correct = y_score + c_score;

        // 5. Simplified Output
        let guess_str = format!("{}{}{}{}", guess[0], guess[1], guess[2], guess[3]);

        println!("--------------------------------------");
        println!("Guess {}: Feedback (D,P) -> {},{}", guess_str, x_total_correct, y_correct_pos);
        println!("--------------------------------------");


        // 6. Check for Win Condition (4 correct positions)
        if y_score == 4 {
            let mut rank_to_assign_final: usize;

            if round_number > last_assigned_round {
                // New, distinct rank
                rank_to_assign_final = rank_to_assign;
                // Prepare rank for the NEXT distinct winner
                rank_to_assign += 1;
            } else {
                // Tie: Assign the rank of the last winner (which is rank_to_assign - 1)
                rank_to_assign_final = rank_to_assign.saturating_sub(1);
                if rank_to_assign_final == 0 {
                    rank_to_assign_final = 1;
                }
            }
            
            // Update the winning round number after assigning the rank
            last_assigned_round = round_number;

            println!("\n🎉🎉🎉 CODE GUESSED! 🎉🎉🎉");
            println!("{} correctly guessed their secret code: {}. They finished in {} place!", 
                     current_player.name, guess_str, rank_to_assign_final);
            
            // Post-Game Menu
            let keep_playing = post_game_menu(&mut players, current_player_index, rank_to_assign_final, &mut completed_players);
            
            if !keep_playing {
                break; // Exit the game loop
            }
            
            // Adjust the current player index since the vector was modified
            current_player_index = current_player_index % players.len(); 
            
            // Clear screen after the menu selection
            clear_screen();
            continue; // Go to the next loop iteration (next player's turn)
        }

        // 7. Pause, clear screen, move to the next player, and check for round completion
        
        println!("\n...Moving to next Player in 5 seconds...");
        thread::sleep(Duration::from_secs(5));
        
        clear_screen();

        // Check if a full round has been completed (total_guesses is a multiple of num_players)
        if total_guesses % (players.len() as u32) == 0 {
            round_number += 1;
        }
        
        current_player_index = (current_player_index + 1) % players.len();
    }
    
    // --- FINAL RANKING DISPLAY ---
    if !completed_players.is_empty() {
        println!("\n======================================");
        println!("|         FINAL RANKINGS         |");
        println!("======================================");
        
        // Sort the players by their assigned rank
        completed_players.sort_by_key(|p| p.rank.unwrap_or(num_players)); 

        for p in completed_players.iter() {
            let rank_str = match p.rank {
                Some(r) => format!("Rank {}", r),
                None => "Unranked".to_string(),
            };
            let secret_str = format!("{}{}{}{}", p.secret_code[0], p.secret_code[1], p.secret_code[2], p.secret_code[3]);
            println!("| {:<15} | {:<8} | Secret: {:<4} |", p.name, rank_str, secret_str);
        }
        println!("======================================");
    }
}

fn main() {
    loop {
        run_game();
        
        // Check if we should restart or quit
        println!("\n--- Game Over ---");
        println!("[1] Start a New Game");
        println!("[2] Quit Program");
        print!("Enter choice (1 or 2): ");
        io::stdout().flush().unwrap();

        match read_line().trim() {
            "1" => {
                // Continue the outer loop to call run_game() again
                clear_screen();
                continue;
            }
            "2" => {
                println!("Thank you for playing! Goodbye.");
                break; // Exit main loop and terminate
            }
            _ => {
                println!("Invalid input. Restarting the menu...");
                thread::sleep(Duration::from_secs(1));
                clear_screen();
            }
        }
    }
}
