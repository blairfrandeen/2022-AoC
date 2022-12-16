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
    // println!("{:?}", file_system);
    for k in file_system.keys() {
        println!("{:?}", k);
    }
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
