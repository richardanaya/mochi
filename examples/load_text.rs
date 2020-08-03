use mochi::*;

fn main() {
    init(include_bytes!("text.gresource"));
    let b = bytes_from_resource("/text/hello.txt");
    let text = std::str::from_utf8(&b).unwrap();
    println!("{}",text);
}
