mod builtin_words;
mod core;
mod test_interface;
mod tty_interface;


use std::io;
use crate::builtin_words::FINAL;

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = true; // atty::is(atty::Stream::Stdout);
    let mut target = String::new();
    io::stdin().read_line(&mut target)?;
    assert!(FINAL.contains(&target.trim().to_lowercase().as_str()));
    let target = target.trim().to_uppercase();
    let target = target.as_str();
    if is_tty {
        tty_interface::tty_mode(target);
    } else {
        test_interface::test_mode(target);
    }
    Ok(())
}
