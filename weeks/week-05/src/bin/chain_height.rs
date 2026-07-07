use std::error::Error;

use week_05_persistence::load_chain_from_file;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: chain_height <chain-json-path>")?;
    let chain = load_chain_from_file(path)?;
    println!("height={}", chain.height());
    Ok(())
}
