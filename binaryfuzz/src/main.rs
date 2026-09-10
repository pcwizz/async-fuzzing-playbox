mod client;
mod fuzzer;
mod instance;
mod target;

use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("../target/debug/testapp").expect("couldn't open test app");

    let mut content = Vec::new();

    file.read_to_end(&mut content)
        .expect("couldn't read program");
    println!("{:?}", content.len());

    let boundaries = target::find_async_foo_boundaries(content.as_slice())
        .expect("couldn't find function boundaries");
}
