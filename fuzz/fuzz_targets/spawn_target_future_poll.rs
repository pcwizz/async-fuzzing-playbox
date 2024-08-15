#![no_main]
#![feature(noop_waker)]

use arbitrary::Arbitrary;
use async_fuzzing_playbox::*;
use core::pin::pin;
use core::task::{Context, Poll, Waker};
use libfuzzer_sys::fuzz_target;
use std::future::{Future, Pending};

#[derive(Arbitrary, Debug)]
struct Input<'a> {
    i: usize,
    s: &'a [u8],
}

fuzz_target!(|data: Input| {
    let fut = pin!(spawn_target(data.i, data.s));
    let mut ctx = Context::from_waker(Waker::noop());

    match fut.poll(&mut ctx) {
        Poll::Ready(_) => return,
        Poll::Pending => unreachable!(),
    }
});