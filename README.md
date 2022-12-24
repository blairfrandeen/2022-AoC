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

# Summary of Puzzles
A reference for me to remember what the puzzles are and what I've learned, and what is the work to go.

## Day 1
Summing grouped lists of numbers. As usual with early AOC problems: conceptually easy, and lots of trouble parsing the input. Solved by storing calorie counts in mutable vector, and then applying `sort()` followed by `reverse()`. Used a slice to get the sum of the top three for part 2.

## Day 2
Rock-paper-scissors. I probably went over-the-top with this one, reating enums for `Rps` and `GameResult`. At the time a good refresher for me in using enums. I think the code ended up very clear, and very verbose at 120+ lines.

## Day 3
Requires splitting strings of characters in the middle, and then finding the letter that appears in both. This one was hard in Rust. The `find_common()` function took some copying from StackOverflow to get right, and took some trickery with `HashSet` that wasn't obvious tome. For part 2 I ended up using `iter.next_chunk()` from the nightly build, although I think others had solutions using `iter.windows()`.

## Day 4
Input is two ranges of numbers, goal is to find ranges that are fully contained by another (part 1) and those that overlap (part 2). In hindsight I think this may have been better to write using `RangeInclusive`, and I actually could have built on it to help with Day 15, where I used ranges extensively.

## Day 5
Moving stacks of containers around. The first part I solved using classic `pop()` and `push()` operations. The second part I rewrote to move entire chunks of vectors around. I ran parts 1 and 2 in the same algorithm, and for part 1 simply reversed the chunks. Parsing was a bit challenging, and in some ways more challenging than the actual algorithm, as there were two sections of puzzle input to parse. Knowing what I know now about `nom` I probably could have done it more cleanly.

## Day 6
This problem involved looking through a string of characters for a "signal" that is made up of some number of unique characters. Part 1 needed 4 characters, part 2 14. I was smart enough to make this a variable so that running part 2 was trivial once part 1 was working. My `find_marker()` algorithm created a new `HashSet` for every window it looked it--would it have been better to update an existing `HashSet` instead?

## Day 7
Building a directory tree and finding sizes of directories. This was the first problem that I felt was getting challenging, and I ended up skipping it for several days. I initially tried to use `nom`, and at one point thought it would be a good candidate for `Rc<T>`, and I ended up using neither.

I ended up solving this by using a `HashMap<String, u32>` as my main data structure, where the keys were the full directory paths, and the values were the sums of the file sizes. In parsing the input I ignored all file names, as well as any output from the `ls` command that showed sub-directories, as those were investigated anyways. It took me a bit to get my own `cd` command working, and getting it to output unique & useful path strings. At the end of buliding the directory map I still had to manually remove the blank directory so I didn't confuse it with root ("/").

## Day 8
Looking at heights of trees in a grid. For this one I started _common.rs_ and made a `Grid` structure, which I later improved upon for Day 12. I ended up having to hard-code the four directions to get the solution for both parts 1 & 2. I would have liked to have an easier & less repetitive way to do it.

## Day 9
Rope mechanics. Figure out all the points that the tail end of a rope (or any point on a rope) touches given the path of the head of the rope. I wrote a simple `Position` struct with X & Y coordinates. Again a good amount of hard-coding for the `follow()` function, when I'm sure there was a way to do it simpler. Part 2 I think required a bit of re-writing, but ended up being the same functions as part 1, simply applied to a different position on the rope.

## Day 10
Given a signal, find where to render pixels on a CRT screen. This one was fun, as the output for part 2 got rendered in santas and christmas trees.

## Day 11
Monkeys throwing items everywhere, with ever-increasing worry. This one was hard, and I didn't get part 2 without looking for some big hints. Without taking the modulus of the increasing worry, the number would overflow. Some parsing challenges as well, which is a reminder how nice it is that the AoC inputs are predictable and error-free. This is the first one where I wrote a function that returns another function using `Box<dyn Fn>`. I still don't understand what the `move` keyword does in this context.

Another flaw in my solution here is that parts 1 & 2 are not distinct; I have to change the code to get one or the other.

## Day 12
A search problem. Given a grid of elevations, find the shortest path from the low point to the high point. Only able to move up one elevation at a time. I re-used and improved upon my `Grid` struct in _common.rs_ for this. I think I came pretty close to getting the solution using BFS without any help, but I screwed up twice. First, I forgot that BFS uses a queue rather than a stack. Second, I spent a long time troubleshooting a BFS algorithm I had copied, because I misread the instructions, and didn't realize that you could move down by any number of stepsthat BFS uses a queue rather than a stack. Second, I spent a long time troubleshooting a BFS algorithm I had copied, because I misread the instructions, and didn't realize that you could move down by any number of steps. RTFM problems.

## Day 13
Unpacking and comparing lists. Stuck on this one. All of my unit tests pass, and I get the right answer for the test input. I think I could go back to using `nom` on this one, and write a recursive algorithm to keep unpacking things until they are comparable. My current strategy is just to convert whatever I get to a vector, and it's not working.

## Day 14
Given a map of a cave, where does all the sand end up if dropped in? Another fun one, with a great ASCII visualization for the output (not requried, but worth it). Getting the cave structure built was challenging since the input only gave me paths of where the rocks were. I had to rewrite some logic from the `Grid` struct, which wasn't worth fully re-using here. 

I had my algorithm for dropping sand return a `Result<(), &str>`, and both parts 1 & 2 kept dropping sand until it returned an error of some sort.

## Day 15
Find the beacon that is sending the distress signal. Given coordinates of sensors and their nearest beacon, where is the one coordinate that isn't covered? I did a lot of manipulating of ranges for this, including merging, truncating, and finding total coverage. 

Part 2 of this puzzle takes an embarassingly long time to run, although it does work. Definitely a candidate for optimization & finding an overall better algorithm than what I came up with.


## Day 18
Finding the exposed surface area of a set of cubes with known coordinates. For part 1, I went through all known points, found how many neighbors each point had, and subtracted that from 6 to get the number of exposed sides.

Part 2 of this also takes embarassingly long to run, since I was only looking for external surfaces. My strategy was to find the inverse of the shape I was given, then break that inverse up into continuous chunks, which essentially used a BFS algorithm, then count the surface area of those chunks, and subtract it from the first answer I got.

## Day 20
Sorting & mixing a ring buffer. Currently stuck, in the process of rewriting so that the original index of an item is tied to the value.

## Day 21
Monkeys yelling numbers at each other, trying to figure out what the "root" monkey yells. I solved part 1 fairly easily using a HashMap. I kept iterating through what was left of the input until the `HashMap` could be fully filled out, and then simply queried the "root" value.

Currently stuck on part 2, where I need to set the value for what "humn" yells to get "root" to yell something intelligible.




