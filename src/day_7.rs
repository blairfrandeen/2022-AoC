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

// use nom::{bytes::complete::tag, IResult};
use std::collections::HashMap;

pub fn main(contents: String) {
    let file_system = build_directory_map(contents);
    let mut sorted_keys: Vec<&String> = file_system.keys().collect();
    let mut dir_sized: HashMap<String, u32> = HashMap::new();
    sorted_keys.sort();
    for index in 0..sorted_keys.len() {
        let mut size: u32 = 0;
        let current_key = sorted_keys[index];
        for next_index in index..sorted_keys.len() {
            let next_key = sorted_keys[next_index];
            if next_key.starts_with(current_key) || current_key == "/" {
                size += file_system[next_key]
            }
        }
        dir_sized.insert(current_key.to_string(), size);
    }
    // println!("{:?}", dir_sized);
    let part_1: u32 = dir_sized.values().filter(|s| *s < &(100000 as u32)).sum();
    println!("Part 1: {}", part_1);
    let fs_size = 70_000_000;
    let required_space = 30_000_000;
    let fs_used: u32 = file_system.values().sum();
    let to_delete = required_space - (fs_size - fs_used);
    let mut candidates: Vec<&u32> = dir_sized.values().filter(|v| v >= &&to_delete).collect();
    candidates.sort();
    // println!("Part 2 FS Used: {}", fs_used);
    // println!("Part 2 To Delete: {}", to_delete);
    println!("Part 2 {:?}", candidates[0]);
}

fn build_directory_map(contents: String) -> HashMap<String, u32> {
    let mut cwd = String::new();
    let mut directory_structure: HashMap<String, u32> = HashMap::new();
    let mut cwd_size: u32 = 0;
    for line in contents.lines() {
        let line_items: Vec<&str> = line.split_whitespace().collect();
        match line_items[0].parse::<u32>() {
            Ok(size) => cwd_size += size,
            Err(_) => {
                if line_items.len() > 2 {
                    // change directory command
                    let key = cwd.clone();
                    if !directory_structure.contains_key(&key) {
                        directory_structure.insert(key, cwd_size);
                    }
                    // reset size counter
                    cwd_size = 0;
                    // get the directory we're in next
                    cwd = cd(line_items[2], &cwd);
                }
            }
        }
    }
    let key = cwd.clone();
    // two not-as-pretty steps to get everything consistent at the end
    directory_structure.insert(format!("{key}"), cwd_size);
    directory_structure.remove("");
    directory_structure
}

fn cd<'a>(dir: &'a str, cwd: &'a str) -> String {
    let mut path: Vec<&str> = cwd.split_terminator('/').collect();
    if dir == ".." {
        path.pop();
    } else {
        path.push(dir);
    }
    if path.len() > 0 && path[0] == "" {
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
        assert_eq!(dirs["/a"], 94269);
        assert_eq!(dirs["/a/e"], 584);
    }
    #[test]
    fn test_cd() {
        assert_eq!(cd("wtf", "/lol"), "/lol/wtf");
        assert_eq!(cd("lol", ""), "/lol");
        assert_eq!(cd("lol", "/"), "/lol");
        assert_eq!(cd("..", "/lol"), "");
    }
}
