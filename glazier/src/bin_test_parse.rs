use glazier::wrl::parser;
use std::fs;

fn main() {
    let walker = walkdir::WalkDir::new("kb").into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
        if entry.path().extension().and_then(|s| s.to_str()) == Some("wrl") {
            let path_str = entry.path().to_string_lossy();
            let content = fs::read_to_string(&*path_str).unwrap();
            match parser::parse_wrl(&content) {
                Ok(prog) => println!("Parsed {}: {} blocks", path_str, prog.blocks.len()),
                Err(e) => println!("Error parsing {}: {:?}", path_str, e),
            }
        }
    }
}
