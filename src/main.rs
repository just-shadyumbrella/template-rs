use mimalloc::MiMalloc;

// Remove this to use the default allocator (slower but smaller binary)
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    println!("Hello, world!");
}
