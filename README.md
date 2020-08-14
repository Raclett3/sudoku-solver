# Sudoku Solver

I will help you solve sudoku! ;)

## Dependency

- Cargo

## Usage

```
$ cargo run --release < sudoku-problem
```

## Problem Format

Numbers should be space-separated and rows should be newline-separated like:

```
8 x x x x x x x x
x x 3 6 x x x x x
x 7 x x 9 x 2 x x
x 5 x x x 7 x x x
x x x x 4 5 7 x x
x x x 1 x x x 3 x
x x 1 x x x x 6 8
x x 8 5 x x x 1 x
x 9 x x x x 4 x x
```

The size should be NxN, where N is a square number.

## How does this solve Sudoku?

Brute-force. This tries placing every possible number.

## License

[MIT](LICENSE)
