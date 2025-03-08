mod client;
mod fuzzer;
mod instance;

use fuzzer::Fuzzer;

fn main() {
    let f = Fuzzer::new();
    f.fuzz().expect("Fuzzer errored");
}
