# terminal-chess

## Essence

A chess game based in the terminal, built in Rust

## How to run

Open up a terminal/command prompt and navigate to the `terminal-chess` directory.

### Through Cargo

To run through cargo, navigate to the `terminal-chess/terminal-chess` directory. Then type `cargo run`.

### Windows

To run on Windows, navigate to the `terminal-chess/windows` directory. Then run the executable by entering `terminal-chess.exe`.

## Limitations & Known Issues

- En Passant and Castling are not considered legal moves.
- Under specific circumstances, the bot will not deliver checkmate, and instead will get stuck in a loop of almost checkmate.
