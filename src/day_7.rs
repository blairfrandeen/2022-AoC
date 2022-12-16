/*
Ok this is a hard one, a lot of parsing to think about
If line starts with '$', it's a command
Only commands are cd and ls
If it's a cd command, that changes the current directory we're in
If it's a ls command, then we get a list of what's in the directory;
Every line until the next command (or EOF) is something that's in the
directory.
If the line starts with 'dir' it's another directory
If it starts with a number then it's a filesize plus a filename.

Do I need to save the filenames? Or can I just add up the file sizes
and assign those sizes to the directory? That would make the problem easier,
however there's a risk that I'll need the file sizes in part 2,
which means I'd regret not grabbing the information to begin with.

This seems like a good use of both the nom crate and the Rc<T> structure.
*/

use std::collections::HashMap;

pub fn main(contents: String) {
    let dir_sized = build_sized_filesystem(build_directory_map(contents));

    let part_1: u32 = dir_sized.values().filter(|s| *s < &100_000).sum();
    println!("Part 1: {part_1}");
    let fs_size = 70_000_000;
    let required_space = 30_000_000;
    let fs_used: u32 = *dir_sized.get("/").unwrap();
    let to_delete = required_space - (fs_size - fs_used);
    let mut candidates: Vec<u32> = dir_sized
        .into_values()
        .filter(|v| *v >= to_delete)
        .collect();
    candidates.sort();
    println!("Part 2: {:?}", candidates[0]);
}

fn build_sized_filesystem(file_system: HashMap<String, u32>) -> HashMap<String, u32> {
    let mut sorted_keys: Vec<&String> = file_system.keys().collect();
    sorted_keys.sort();
    let mut dir_sized: HashMap<String, u32> = HashMap::new();
    for index in 0..sorted_keys.len() {
        let mut current_size: u32 = 0;
        let current_key = sorted_keys[index];
        for next_key in sorted_keys.iter().skip(index) {
            if next_key.starts_with(current_key) || current_key == "/" {
                current_size += file_system[*next_key]
            }
        }
        dir_sized.insert(current_key.to_string(), current_size);
    }
    dir_sized
}

fn build_directory_map(contents: String) -> HashMap<String, u32> {
    let mut cwd = String::new(); // current working directory
    let mut cwd_size: u32 = 0; // total size of files in cwd
    let mut directory_structure: HashMap<String, u32> = HashMap::new();
    for line in contents.lines() {
        let line_words: Vec<&str> = line.split_whitespace().collect();
        match line_words[0].parse::<u32>() {
            // if first word is a number, it's a file size
            Ok(size) => cwd_size += size,
            Err(_) => {
                if line_words.len() > 2 {
                    // change directory command
                    if !directory_structure.contains_key(&cwd) {
                        directory_structure.insert(cwd.clone(), cwd_size);
                    }
                    // reset size counter
                    cwd_size = 0;
                    // get the directory we're in next
                    cwd = cd(line_words[2], &cwd);
                }
            }
        }
    }
    // two not-as-pretty steps to get everything consistent at the end
    directory_structure.insert(cwd, cwd_size);
    directory_structure.remove("");
    directory_structure
}

fn cd<'a>(dir: &'a str, cwd: &'a str) -> String {
    // Change directories to `dir` from `cwd`.
    // `cwd` is a string separated by "/"
    // Passing `cwd=".."` moves up a level
    let mut path: Vec<&str> = cwd.split_terminator('/').collect();
    if dir == ".." {
        path.pop();
    } else {
        path.push(dir);
    }
    if !path.is_empty() && path[0].is_empty() {
        path.remove(0);
    }
    path.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_map() {
        let input = fs::read_to_string("inputs/2022.7.test").unwrap();
        let dirs = build_directory_map(input);
        assert_eq!(dirs["a"], 94269);
        assert_eq!(dirs["a/e"], 584);
    }
    #[test]
    fn test_cd() {
        assert_eq!(cd("wtf", "/lol"), "lol/wtf");
        assert_eq!(cd("lol", ""), "lol");
        assert_eq!(cd("lol", "/"), "lol");
        assert_eq!(cd("..", "/lol"), "");
    }
}
