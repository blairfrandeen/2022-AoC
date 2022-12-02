pub fn main(contents: String) {
    println!("Hello AoC!")
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
