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