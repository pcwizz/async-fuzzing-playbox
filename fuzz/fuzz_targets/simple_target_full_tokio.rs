#![no_main]

use arbitrary::Arbitrary;
use async_fuzzing_playbox::*;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct Input<'a> {
    i: usize,
    s: &'a [u8],
}

fuzz_target!(|data: Input| {
    // Build a new tokio runtime for every fuzz exec
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let _ = simple_target(data.i, data.s).await;
        })
});
