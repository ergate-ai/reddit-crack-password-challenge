#!/usr/bin/env python3
import itertools
import time

def load_bip39_from_file(filename):
    """
    Loads the BIP39 word list from a local file.
    Expects a plain text file with one word per line.
    """
    with open(filename, "r") as f:
        words = [line.strip() for line in f if line.strip()]
    return words

def crack_seed(target_seed, bip39_words):
    """
    Brute-force search for the target seed by generating all possible 12-word sequences.
    
    WARNING: The total number of combinations (2048^12) is computationally infeasible to fully iterate.
    This code is for demonstration only.
    """
    target_seed_tuple = tuple(target_seed)
    attempts = 0
    start_time = time.time()

    # Iterate over all 12-word combinations (Cartesian product)
    for candidate in itertools.product(bip39_words, repeat=12):
        attempts += 1
        if candidate == target_seed_tuple:
            elapsed = time.time() - start_time
            print("Found target seed:")
            print(" ".join(candidate))
            print(f"Attempts: {attempts}")
            print(f"Time taken: {elapsed:.2f} seconds")
            return
        # Print status every 1,000,000 attempts for demonstration purposes
        if attempts % 1_000_000 == 0:
            elapsed = time.time() - start_time
            print(f"{attempts} attempts so far, elapsed time: {elapsed:.2f} seconds")
            print("Last checked:", " ".join(candidate), "\n")

    print("Target seed not found.")

if __name__ == "__main__":
    # Load the BIP39 word list from the local file "english.txt"
    filename = "english.txt"
    print("Loading BIP39 word list from file:", filename)
    bip39_words = load_bip39_from_file(filename)
    print("Total BIP39 words loaded:", len(bip39_words))

    # Hardcoded target seed – must be 12 words from the BIP39 list
    target_seed = "abandon ability able about above absent absorb abstract absurd abuse access accident".split()

    print("Starting brute-force search for the target seed...")
    crack_seed(target_seed, bip39_words)
