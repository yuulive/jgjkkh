use std::io::Write;

/// Does a readline from stdin and returns a trimmed string
/// # Arguments
///
/// * `prompt` - A string slice that is displayed as a prompt to the user
///
/// # Examples
///
/// ```
/// let name = input("What's your name? : ");
/// println!("Oh! So your name is {}!", name);
/// ```
pub fn input(prompt: &str) -> Result<String, std::io::Error> {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut result = String::new();
    match std::io::stdin().read_line(&mut result) {
        Ok(_) => Ok(result.trim().to_string()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
