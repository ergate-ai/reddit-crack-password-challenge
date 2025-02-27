# BIP39 Seed Phrase Brute-Force Challenge

I had another sub thread going on where a few smart pants claimed you could crack a 12-word password using a dictionary attack "easily"". Now, here’s the challenge: see if you can optimize this code and crack the password already given in the code by using a brute force + dictionary attack approach!

Some more background, I use 52 character long password for BitWarden and people claimed that since I shared my password length - I was stupid and now anyone can crack my password. 

Here is the original thread for context

https://aww.sm/4DaROLK

## Overview

This repository contains a simple Python script that:
- Loads the BIP39 English word list directly from local english.txt (https://raw.githubusercontent.com/bitcoin/bips/refs/heads/master/bip-0039/english.txt)
- Uses the word list as a dictionary to generate all possible 12-word mnemonic combinations.
- Compares each generated candidate against a hardcoded target seed phrase `abandon ability able about above absent absorb abstract absurd abuse access accident`

**Note:** The complete search space (approximately 2048^12 combinations) is astronomically huge and this script is intended purely for demonstration purposes. It will not complete in any realistic timeframe using naive brute force. Even though the seed phrase is using all initial words of the list its still waste of CPU resources to attempt to crack it

## The Challenge

- **Objective:** Optimize the provided code to efficiently search through the 12-word combination space and crack the target seed.
- **Task:** Implement improvements—whether through smarter heuristics, parallel processing, or any algorithmic enhancements—to reduce the time needed to find the matching seed phrase.
- **Target:** The seed phrase to crack is already hardcoded in the script for testing purposes.

## Disclaimer

This project is for **educational and research purposes only**. Do **not** use this code to attempt unauthorized access or compromise any real wallets or systems. Always work within legal boundaries and in controlled environments with explicit permission.

This is an example of the speed I was getting running it on MacBook Pro M4 Max

![alt text](<https://github.com/ergate-ai/reddit-crack-password-challenge/blob/main/screenshots/example.png?raw=true>)


## How to Get Started

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/ergate-ai/reddit-crack-password-challenge.git
   cd reddit-crack-password-challenge
   ```

2. **Run the Scrip:**
   ```bash
   python3 main.py
   ```

3. **Optimize and Contribute:**
Fork the repository, implement your optimizations, and open a pull request with your improvements. Let’s see who can bring this challenge down to a manageable timeframe!

## Did you say Rust is better ?

There is a rust version in the repo now. Go to ./rust folder and do `cargo run`

The comparision results when ran on same computer are infact better for Python. 70 seconds for billion password in Python and 80 seconds for billion password in Rust


This is for Rust
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rust`
Loading BIP39 word list from file: english.txt
Total BIP39 words loaded: 2048
Starting brute-force search for the target seed...
100000000 attempts so far, elapsed time: 8.06 seconds
200000000 attempts so far, elapsed time: 16.20 seconds
300000000 attempts so far, elapsed time: 24.34 seconds
400000000 attempts so far, elapsed time: 32.53 seconds
500000000 attempts so far, elapsed time: 40.70 seconds
600000000 attempts so far, elapsed time: 48.89 seconds
```

This is for Python
```
python3 main.py
Loading BIP39 word list from file: english.txt
Total BIP39 words loaded: 2048
Starting brute-force search for the target seed...
100000000 attempts so far, elapsed time: 7.08 seconds
200000000 attempts so far, elapsed time: 14.13 seconds
300000000 attempts so far, elapsed time: 21.19 seconds
400000000 attempts so far, elapsed time: 28.28 seconds
500000000 attempts so far, elapsed time: 35.35 seconds
600000000 attempts so far, elapsed time: 42.45 seconds
```



## License
This project is licensed under the MIT License.

