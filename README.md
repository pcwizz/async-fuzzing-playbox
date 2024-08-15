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

## Reading target manually executing future

```
#2      INITED cov: 213 ft: 213 corp: 1/1b exec/s: 0 rss: 54Mb
#432    NEW    cov: 224 ft: 224 corp: 2/8b lim: 8 exec/s: 0 rss: 60Mb L: 7/7 MS: 5 InsertByte-InsertByte-EraseBytes-CrossOver-InsertRepeatedBytes-
#759    NEW    cov: 226 ft: 240 corp: 3/19b lim: 11 exec/s: 0 rss: 65Mb L: 11/11 MS: 2 ChangeBinInt-InsertRepeatedBytes-
#1792   NEW    cov: 226 ft: 256 corp: 4/40b lim: 21 exec/s: 0 rss: 79Mb L: 21/21 MS: 3 InsertRepeatedBytes-ChangeBinInt-InsertRepeatedBytes-
#3033   NEW    cov: 226 ft: 259 corp: 5/72b lim: 33 exec/s: 0 rss: 96Mb L: 32/32 MS: 1 InsertRepeatedBytes-
#3417   REDUCE cov: 226 ft: 259 corp: 5/71b lim: 33 exec/s: 0 rss: 101Mb L: 31/31 MS: 4 InsertByte-ShuffleBytes-ChangeBit-EraseBytes-
#4530   NEW    cov: 226 ft: 261 corp: 6/112b lim: 43 exec/s: 0 rss: 117Mb L: 41/41 MS: 3 InsertByte-CopyPart-InsertByte-
#6582   NEW    cov: 226 ft: 274 corp: 7/175b lim: 63 exec/s: 0 rss: 145Mb L: 63/63 MS: 2 CopyPart-InsertRepeatedBytes-
#6587   REDUCE cov: 226 ft: 274 corp: 7/173b lim: 63 exec/s: 0 rss: 145Mb L: 61/61 MS: 5 CopyPart-EraseBytes-ChangeBit-CopyPart-InsertRepeatedBytes-
#7771   NEW    cov: 226 ft: 275 corp: 8/244b lim: 74 exec/s: 0 rss: 161Mb L: 71/71 MS: 4 ChangeBinInt-EraseBytes-InsertByte-InsertRepeatedBytes-
#9019   NEW    cov: 226 ft: 277 corp: 9/327b lim: 86 exec/s: 0 rss: 178Mb L: 83/83 MS: 3 ChangeBinInt-CrossOver-InsertRepeatedBytes-
#9515   REDUCE cov: 226 ft: 277 corp: 9/325b lim: 86 exec/s: 0 rss: 185Mb L: 81/81 MS: 1 CrossOver-
#15270  REDUCE cov: 226 ft: 290 corp: 10/466b lim: 142 exec/s: 0 rss: 264Mb L: 141/141 MS: 5 InsertRepeatedBytes-ChangeBinInt-CrossOver-ShuffleBytes-InsertByte-
#16690  NEW    cov: 226 ft: 291 corp: 11/617b lim: 156 exec/s: 0 rss: 284Mb L: 151/151 MS: 5 ChangeBinInt-ChangeByte-CMP-ChangeBinInt-CrossOver- DE: "\012\000\000\000\000\000\000\000"-
#17397  NEW    cov: 226 ft: 293 corp: 12/780b lim: 163 exec/s: 0 rss: 294Mb L: 163/163 MS: 2 CrossOver-InsertRepeatedBytes-
#21390  REDUCE cov: 226 ft: 293 corp: 12/778b lim: 198 exec/s: 0 rss: 350Mb L: 161/161 MS: 3 PersAutoDict-InsertRepeatedBytes-EraseBytes- DE: "\012\000\000\000\000\000\000\000"-
#32852  NEW    cov: 226 ft: 306 corp: 13/1085b lim: 309 exec/s: 0 rss: 423Mb L: 307/307 MS: 2 CopyPart-InsertRepeatedBytes-
#32885  REDUCE cov: 226 ft: 306 corp: 13/1083b lim: 309 exec/s: 0 rss: 423Mb L: 305/305 MS: 3 ChangeBinInt-ShuffleBytes-CrossOver-
#33470  REDUCE cov: 226 ft: 306 corp: 13/1082b lim: 309 exec/s: 0 rss: 424Mb L: 304/304 MS: 5 InsertByte-ChangeByte-EraseBytes-ChangeByte-InsertRepeatedBytes-
#34332  REDUCE cov: 226 ft: 307 corp: 14/1394b lim: 317 exec/s: 0 rss: 424Mb L: 312/312 MS: 2 ChangeBit-InsertRepeatedBytes-
#34552  REDUCE cov: 226 ft: 307 corp: 14/1393b lim: 317 exec/s: 0 rss: 424Mb L: 303/312 MS: 5 PersAutoDict-CrossOver-ChangeBit-ShuffleBytes-CrossOver- DE: "\012\000\000\000\000\000\000\000"-
#35485  NEW    cov: 226 ft: 309 corp: 15/1715b lim: 325 exec/s: 0 rss: 425Mb L: 322/322 MS: 3 CopyPart-ShuffleBytes-CrossOver-
#43093  REDUCE cov: 226 ft: 309 corp: 15/1714b lim: 397 exec/s: 0 rss: 428Mb L: 321/321 MS: 3 InsertRepeatedBytes-ShuffleBytes-EraseBytes-
#47597  REDUCE cov: 226 ft: 309 corp: 15/1712b lim: 437 exec/s: 0 rss: 429Mb L: 301/321 MS: 4 ChangeBit-ChangeBit-CMP-EraseBytes- DE: "\005\000\000\000\000\000\000\000"-
#74122  REDUCE cov: 226 ft: 309 corp: 15/1711b lim: 697 exec/s: 74122 rss: 434Mb L: 311/321 MS: 5 CopyPart-EraseBytes-PersAutoDict-InsertRepeatedBytes-CrossOver- DE: "\005\000\000\000\000\000\000\000"-
#163491 NEW    cov: 226 ft: 325 corp: 16/3214b lim: 1580 exec/s: 81745 rss: 435Mb L: 1503/1503 MS: 4 CrossOver-ChangeBit-CopyPart-CopyPart-
#163674 REDUCE cov: 226 ft: 325 corp: 16/3128b lim: 1580 exec/s: 81837 rss: 435Mb L: 1417/1417 MS: 3 InsertByte-InsertByte-EraseBytes-
#164458 REDUCE cov: 226 ft: 325 corp: 16/3042b lim: 1580 exec/s: 82229 rss: 435Mb L: 1331/1331 MS: 4 ChangeBit-CopyPart-InsertByte-CrossOver-
#164509 REDUCE cov: 226 ft: 325 corp: 16/3025b lim: 1580 exec/s: 82254 rss: 435Mb L: 1314/1314 MS: 1 EraseBytes-
#165096 REDUCE cov: 226 ft: 325 corp: 16/3004b lim: 1580 exec/s: 82548 rss: 435Mb L: 1293/1293 MS: 2 ChangeByte-EraseBytes-
#169699 REDUCE cov: 226 ft: 325 corp: 16/3001b lim: 1620 exec/s: 84849 rss: 436Mb L: 1290/1290 MS: 3 ChangeASCIIInt-ShuffleBytes-EraseBytes-
#176168 REDUCE cov: 226 ft: 325 corp: 16/2997b lim: 1680 exec/s: 88084 rss: 438Mb L: 1286/1286 MS: 4 EraseBytes-PersAutoDict-CopyPart-InsertRepeatedBytes- DE: "\012\000\000\000\000\000\000\000"-
#203915 REDUCE cov: 226 ft: 325 corp: 16/2992b lim: 1950 exec/s: 101957 rss: 442Mb L: 1281/1281 MS: 2 PersAutoDict-EraseBytes- DE: "\005\000\000\000\000\000\000\000"-
#262144 pulse  cov: 226 ft: 325 corp: 16/2992b lim: 2523 exec/s: 87381 rss: 444Mb
#384989 REDUCE cov: 226 ft: 338 corp: 17/5552b lim: 3744 exec/s: 76997 rss: 445Mb L: 2560/2560 MS: 4 CopyPart-InsertRepeatedBytes-ChangeASCIIInt-InsertRepeatedBytes-
#437983 REDUCE cov: 226 ft: 338 corp: 17/5548b lim: 4096 exec/s: 72997 rss: 462Mb L: 2556/2556 MS: 4 PersAutoDict-CopyPart-ChangeBinInt-EraseBytes- DE: "\005\000\000\000\000\000\000\000"-
#475249 REDUCE cov: 226 ft: 338 corp: 17/5546b lim: 4096 exec/s: 79208 rss: 463Mb L: 2554/2554 MS: 1 EraseBytes-
#484309 REDUCE cov: 226 ft: 338 corp: 17/5543b lim: 4096 exec/s: 80718 rss: 463Mb L: 2551/2551 MS: 5 EraseBytes-InsertByte-ChangeASCIIInt-PersAutoDict-CopyPart- DE: "\012\000\000\000\000\000\000\000"-
#524288 pulse  cov: 226 ft: 338 corp: 17/5543b lim: 4096 exec/s: 74898 rss: 464Mb
#1048576        pulse  cov: 226 ft: 338 corp: 17/5543b lim: 4096 exec/s: 74898 rss: 465Mb
#2097152        pulse  cov: 226 ft: 338 corp: 17/5543b lim: 4096 exec/s: 72315 rss: 465Mb
#4194304        pulse  cov: 226 ft: 338 corp: 17/5543b lim: 4096 exec/s: 69905 rss: 467Mb
```

When functions stick to operating on traits like `AsyncRead` then we can still
poll the futures to completion manually. We maintain minimal performance
penality for the code bing async and all is good.