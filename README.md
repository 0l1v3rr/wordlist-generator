# Wordlist Generator
This is a very simple wordlist generator written in Rust.<br>
This program can be used to generate possible passwords by entering a few words.

## Setup
Download and install [Rust](https://www.rust-lang.org/learn/get-started)<br>
Then clone this repo:
```sh
git clone https://github.com/0l1v3rr/wordlist-generator.git
cd wordlist-generator
```
Now you can run the program with the following command:
```
cargo run
```

## Example output
```
Enter the words separated by semicolons: josh;john;2002;1991
Enter the name of the wordlist: test
Generating words...
Done! Generated words: 520 words
```
Sample from **test.txt** file:
```
josh2002
josh-2002
josh_2002
josh%2002
josh/2002
2002josh
2002-josh
2002_josh
2002%josh
...
```