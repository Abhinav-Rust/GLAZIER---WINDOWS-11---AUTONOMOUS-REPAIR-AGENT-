use glazier::wrl::parser;
use std::fs;

fn main() {
    let content = fs::read_to_string("glazier/kb/audio/microphone.wrl").unwrap();
    match parser::parse_wrl(&content) {
        Ok(prog) => println!("Success! {:?}", prog.blocks.len()),
        Err(e) => println!("Error: {:?}", e),
    }
}
