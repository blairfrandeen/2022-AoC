# Advent of Code 2022
Wherein I learn me some Rust.

## Helper Script Instructions
To download puzzle input and add module for a given day, simply run
```shell
python aochelper.py 2022 X
```
where X is the day. This will:
- Download puzzle input and save it in `inputs/2022.x`.
- Create `src/day_x.rs` based on `src/day_template.rs`.
- Add `pub mod day_x;` to the end of `src/main.rs`.
- Add `x ==> day_x::main(contents)` to switch statement in `src/main.rs`.

Puzzle input is downloaded by finding the Advent of Code login cookie from Firefox. Works in Windows with WSL, probably doesn't work elsewhere.

## Running Puzzle
Run code using `cargo run 2022 X` where X is the day. Using `cargo run 2022 X test` to run from the `inputs/2022.x.test` file.
