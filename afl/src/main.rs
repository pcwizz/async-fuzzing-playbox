#[macro_use]
extern crate afl;

use arbitrary::Arbitrary;
use async_fuzzing_playbox::*;

#[derive(Arbitrary, Debug)]
struct Input<'a> {
    i: usize,
    s: &'a [u8],
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    fuzz!(|data: Input| {
        // Build a new tokio runtime for every fuzz exec
        runtime.block_on(async {
            let _ = simple_target(data.i, data.s).await;
        })
    });
}
