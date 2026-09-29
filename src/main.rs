use const_format::concatcp;
use mimalloc::MiMalloc;
use std::env;

const PERSON: &str = "Bob";
const GREETING: &str = "Welcome ";

const MESSAGE: &str = concatcp!(GREETING, PERSON, "!");

// Remove this to use the default allocator (slower but smaller binary)
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    println!("Hello, world!");
    println!(
        "{} a.k.a {}...",
        MESSAGE,
        env::var("USERNAME").unwrap_or_default()
    );
}
