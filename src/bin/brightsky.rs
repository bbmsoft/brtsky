use brightsky_rs::*;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let data: Response = serde_json::from_str(&buffer)?;

    println!("{}", data);

    Ok(())
}
