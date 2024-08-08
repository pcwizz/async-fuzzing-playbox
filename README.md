# Async fuzz playbox

## Simple Target full tokio

```
#422    NEW    cov: 614 ft: 614 corp: 2/9b lim: 8 exec/s: 0 rss: 58Mb L: 8/8 MS: 4 ShuffleBytes-CrossOver-InsertRepeatedBytes-CopyPart-
#423    NEW    cov: 619 ft: 632 corp: 3/17b lim: 8 exec/s: 0 rss: 58Mb L: 8/8 MS: 1 ChangeByte-
#131072 pulse  cov: 619 ft: 632 corp: 3/17b lim: 1300 exec/s: 21845 rss: 263Mb
#262144 pulse  cov: 619 ft: 632 corp: 3/17b lim: 2600 exec/s: 29127 rss: 263Mb
#524288 pulse  cov: 619 ft: 632 corp: 3/17b lim: 4096 exec/s: 37449 rss: 263Mb
#1048576        pulse  cov: 619 ft: 632 corp: 3/17b lim: 4096 exec/s: 43690 rss: 263Mb
#2097152        pulse  cov: 619 ft: 632 corp: 3/17b lim: 4096 exec/s: 47662 rss: 263Mb
#4194304        pulse  cov: 619 ft: 632 corp: 3/17b lim: 4096 exec/s: 48770 rss: 263Mb
#8388608        pulse  cov: 619 ft: 632 corp: 3/17b lim: 4096 exec/s: 49932 rss: 263Mb
```

## Simple Target manually executing future

```
#417	NEW    cov: 27 ft: 27 corp: 2/9b lim: 8 exec/s: 0 rss: 53Mb L: 8/8 MS: 5 ChangeBit-CMP-CrossOver-CopyPart-InsertByte- DE: "\001\000\000\000"-
#4194304	pulse  cov: 27 ft: 27 corp: 2/9b lim: 4096 exec/s: 1398101 rss: 357Mb
```

Roughly a 90x speedup in terms of executions per second compared to setting up and running tokio each time.

## Simple Target full tokio lazy

```
414    NEW    cov: 378 ft: 378 corp: 2/9b lim: 8 exec/s: 0 rss: 57Mb L: 8/8 MS: 1 InsertRepeatedBytes-
#435    NEW    cov: 383 ft: 393 corp: 3/17b lim: 8 exec/s: 0 rss: 57Mb L: 8/8 MS: 1 CrossOver-
#524288 pulse  cov: 383 ft: 393 corp: 3/17b lim: 4096 exec/s: 131072 rss: 96Mb
#1048576        pulse  cov: 383 ft: 393 corp: 3/17b lim: 4096 exec/s: 149796 rss: 135Mb
#16777216       pulse  cov: 383 ft: 393 corp: 3/17b lim: 4096 exec/s: 143395 rss: 591Mb
#33554432       pulse  cov: 383 ft: 393 corp: 3/17b lim: 4096 exec/s: 142784 rss: 592Mb
```

Using a full tokio runtime but using `lazy_static` to only build the tokio
runtime once to save some overhead on each exec. This delivers about a 3x
speedup in terms of executions per second compared with creating a new tokio
runtime each time. But is still significantly slower than manually polling a
future until ready.

This does introduce the possibility of leaking tasks and timers onto the runtime
which could impact the reproducibility of each fuzz run. For the simple target
we know that it does not spawn any tasks or timers, although for this simple
target we also know we don't need tokio, for more realistically complex targets
it may be reasonable to accept the uncertainty.

The libfuzzer interface itself does expose an init function that we could use to
build the runtime, however the rust bindings do not currently exposes this.
`lazy_static` is a merely an easy workaround. The AFL bindings are designed to
embed the fuzz target inside a main function where we could do this setup before
the fuzz target so that we avoid repeating it as we do here with `lazy_static`
but whilst also saving the small runtime cost of `lazy_static` `Deref` in each
execution. 
