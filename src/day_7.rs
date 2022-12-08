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

use nom::{bytes::complete::tag, IResult};
pub fn main(contents: String) {
    for line in contents.lines() {
        parse_line(&line);
    }
}

fn parse_line(line: &str) -> IResult<&str, &str> {
    let (cmd, _) = tag("$ ")(line)?;
    println!("{} {}", line, cmd);

    Ok((line, "success"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_42() {
        let answer = 42;
        assert_eq!(answer, 42)
    }
}
