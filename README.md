# Advent of Code 2022
Wherein I learn me some Rust.

## Getting Puzzle inputs
Simply run
```shell
python aochelper.py 2022 X
```
where X is the day. This will download puzzle input and save it in `inputs/2022.x`

## Instructions to myself for adding modules / days:
1. For each day, add `src/day_x.rs`
2. Library for each day needs a `pub fn main(contents: String)`
3. In `src/main.rs`, add `pub mod day_x`
4. Solve puzzle in `src/day_x.rs`
5. Run code using `cargo run 2022 X` where X is the day. Using `cargo run 2022 X test` to run from the `inputs/2022.x.test` file.
