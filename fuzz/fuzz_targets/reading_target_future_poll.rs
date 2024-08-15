#![no_main]
#![feature(noop_waker)]

use async_fuzzing_playbox::*;
use core::pin::pin;
use core::task::{Context, Poll, Waker};
use libfuzzer_sys::fuzz_target;
use std::future::{Future, Pending};

fuzz_target!(|data: &[u8]| {
    let reader = tokio_test::io::Builder::new().read(data).build();
    let fut = pin!(reading_target(reader));
    let mut ctx = Context::from_waker(Waker::noop());

    match fut.poll(&mut ctx) {
        Poll::Ready(_) => return,
        Poll::Pending => unreachable!(),
    }
});
