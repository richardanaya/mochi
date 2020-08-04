use mochi::*;

fn main() {
    init(include_bytes!("resources/text.gresource"));
    let text = text_from_resource("/text/hello.txt");
    println!("{}", text);
}
