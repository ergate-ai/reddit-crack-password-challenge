use std::fs;
use std::io;
use std::time::Instant;

/// Loads the BIP39 word list from a local file.
/// Expects a plain text file with one word per line.
fn load_bip39_from_file(filename: &str) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(filename)?;
    let words = content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    Ok(words)
}

/// Recursively generates all possible 12-word combinations and checks if a candidate
/// matches the target seed. The search space is huge (2048^12 combinations).
fn crack_seed(target_seed: &Vec<String>, bip39_words: &Vec<String>) {
    let mut attempts: u64 = 0;
    let start_time = Instant::now();
    let mut found = false;
    let mut candidate: Vec<String> = Vec::with_capacity(12);

    fn rec(
        candidate: &mut Vec<String>,
        depth: usize,
        target_seed: &Vec<String>,
        bip39_words: &Vec<String>,
        attempts: &mut u64,
        start_time: &Instant,
        found: &mut bool,
    ) {
        if *found {
            return;
        }
        if depth == 12 {
            *attempts += 1;
            if candidate == target_seed {
                let elapsed = start_time.elapsed().as_secs_f64();
                println!("Found target seed:");
                println!("{}", candidate.join(" "));
                println!("Attempts: {}", attempts);
                println!("Time taken: {:.2} seconds", elapsed);
                *found = true;
            }
            if *attempts % 100_000_000 == 0 {
                let elapsed = start_time.elapsed().as_secs_f64();
                println!(
                    "{} attempts so far, elapsed time: {:.2} seconds",
                    attempts, elapsed
                );
            }
            return;
        } else {
            for word in bip39_words {
                candidate.push(word.clone());
                rec(candidate, depth + 1, target_seed, bip39_words, attempts, start_time, found);
                if *found {
                    return;
                }
                candidate.pop();
            }
        }
    }

    rec(&mut candidate, 0, target_seed, bip39_words, &mut attempts, &start_time, &mut found);

    if !found {
        println!("Target seed not found.");
    }
}

fn main() {
    // Load the BIP39 word list from the local file "english.txt"
    let filename = "english.txt";
    println!("Loading BIP39 word list from file: {}", filename);
    let bip39_words = load_bip39_from_file(filename)
        .expect("Failed to load BIP39 word list");
    println!("Total BIP39 words loaded: {}", bip39_words.len());

    // Hardcoded target seed – must be 12 words from the BIP39 list
    let target_seed_str = "abandon ability able about above absent absorb abstract absurd abuse access accident";
    let target_seed: Vec<String> = target_seed_str
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    println!("Starting brute-force search for the target seed...");
    crack_seed(&target_seed, &bip39_words);
}
