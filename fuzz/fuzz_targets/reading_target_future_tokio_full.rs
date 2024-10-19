#![no_main]
#![feature(noop_waker)]

use async_fuzzing_playbox::*;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let reader = tokio_test::io::Builder::new().read(data).build();

    tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
        let _ = reading_target(reader).await;
    })
});
