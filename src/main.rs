use std::env;
use std::path::Path;

fn main() {
    let filename = env::args()
        .skip(1).next()
        .expect("Usage: vlc-module-tool <filename>");

    let filepath = Path::new(&filename);

    println!("Opening library {}", filename);
}
